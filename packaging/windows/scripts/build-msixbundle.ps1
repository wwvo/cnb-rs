[CmdletBinding()]
param(
    [string]$X64ContentDir = "target/windows-msix/appxcontent/x64",
    [string]$Arm64ContentDir = "target/windows-msix/appxcontent/arm64",
    [string]$OutputDir = "target/windows-msix/output",
    [string]$CliPath = "target/tools/msixbundle-cli/bin/msixbundle-cli.exe",
    [string]$PfxPath,
    [string]$PfxPassword,
    [string]$CertificatePath,
    [string]$Version,
    [string]$PackageBase = "cnb-rs"
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

function Get-RepoRoot {
    $scriptDir = $PSScriptRoot
    return [System.IO.Path]::GetFullPath((Join-Path $scriptDir "..\..\.."))
}

function Resolve-FullPath {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Path,

        [Parameter(Mandatory = $true)]
        [string]$BasePath
    )

    if ([System.IO.Path]::IsPathRooted($Path)) {
        return [System.IO.Path]::GetFullPath($Path)
    }

    return [System.IO.Path]::GetFullPath((Join-Path $BasePath $Path))
}

function Get-WorkspaceVersion {
    param(
        [Parameter(Mandatory = $true)]
        [string]$ManifestPath
    )

    $manifest = Get-Content $ManifestPath -Raw
    $match = [regex]::Match($manifest, '(?m)^\s*version\s*=\s*"(?<version>\d+\.\d+\.\d+(?:[-+][^"]+)?)"\s*$')
    if (-not $match.Success) {
        throw "Failed to detect workspace version from $ManifestPath"
    }

    return $match.Groups["version"].Value
}

function Convert-ToMsixVersion {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Semver
    )

    $match = [regex]::Match($Semver, '^(?<major>\d+)\.(?<minor>\d+)\.(?<patch>\d+)')
    if (-not $match.Success) {
        throw "Unsupported workspace version for MSIX packaging: $Semver"
    }

    return "{0}.{1}.{2}.0" -f $match.Groups["major"].Value, $match.Groups["minor"].Value, $match.Groups["patch"].Value
}

function Write-StepLog {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Message
    )

    $timestamp = (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ssZ")
    Write-Host ("[{0}] {1}" -f $timestamp, $Message)
}

function Import-TemporaryTrustedCertificate {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Path
    )

    $cert = New-Object System.Security.Cryptography.X509Certificates.X509Certificate2($Path)
    $addedStores = @()
    Write-StepLog "Preparing temporary trust import for certificate $($cert.Thumbprint) from $Path"
    # Avoid Root stores here because they can trigger blocking trust prompts on headless runners.
    foreach ($storePath in @(
        "Cert:\CurrentUser\TrustedPeople",
        "Cert:\LocalMachine\TrustedPeople"
    )) {
        Write-StepLog "Checking certificate store $storePath"
        $existing = Get-ChildItem $storePath | Where-Object Thumbprint -eq $cert.Thumbprint | Select-Object -First 1
        if (-not $existing) {
            Write-StepLog "Importing certificate into $storePath"
            Import-Certificate -FilePath $Path -CertStoreLocation $storePath | Out-Null
            $addedStores += $storePath
            Write-StepLog "Imported certificate into $storePath"
        }
        else {
            Write-StepLog "Certificate already present in $storePath"
        }
    }

    return [pscustomobject]@{
        Thumbprint = $cert.Thumbprint
        AddedStores = $addedStores
    }
}

function Import-TemporarySigningCertificate {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Path,

        [string]$Password,

        [string]$StorePath = "Cert:\CurrentUser\My"
    )

    $securePassword = ConvertTo-SecureString -String $Password -AsPlainText -Force
    $certificate = Get-PfxCertificate -FilePath $Path -Password $securePassword
    if (-not $certificate) {
        throw "Failed to read signing certificate from PFX: $Path"
    }

    Write-StepLog "Preparing temporary signing certificate import for thumbprint $($certificate.Thumbprint) into $StorePath"
    $existing = Get-ChildItem $StorePath | Where-Object Thumbprint -eq $certificate.Thumbprint | Select-Object -First 1
    $addedStores = @()
    if (-not $existing) {
        Write-StepLog "Importing PFX into $StorePath for SignTool access"
        Import-PfxCertificate -FilePath $Path -Password $securePassword -CertStoreLocation $StorePath | Out-Null
        $addedStores += $StorePath
        Write-StepLog "Imported PFX into $StorePath"
    }
    else {
        Write-StepLog "Signing certificate already present in $StorePath"
    }

    return [pscustomobject]@{
        Thumbprint = $certificate.Thumbprint
        AddedStores = $addedStores
        CertStore = "My"
        UseMachineStore = $false
    }
}

function Remove-TemporaryTrustedCertificate {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Thumbprint,

        [string[]]$StorePaths = @(
            "Cert:\CurrentUser\TrustedPeople",
            "Cert:\LocalMachine\TrustedPeople"
        )
    )

    foreach ($storePath in $StorePaths) {
        $matchingCertificates = Get-ChildItem $storePath | Where-Object Thumbprint -eq $Thumbprint
        if ($matchingCertificates) {
            Write-StepLog "Removing temporary certificate $Thumbprint from $storePath"
            $matchingCertificates | Remove-Item -Force -ErrorAction SilentlyContinue
        }
    }
}

function Remove-TemporarySigningCertificate {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Thumbprint,

        [string[]]$StorePaths = @("Cert:\CurrentUser\My")
    )

    foreach ($storePath in $StorePaths) {
        $matchingCertificates = Get-ChildItem $storePath | Where-Object Thumbprint -eq $Thumbprint
        if ($matchingCertificates) {
            Write-StepLog "Removing temporary signing certificate $Thumbprint from $storePath"
            $matchingCertificates | Remove-Item -Force -ErrorAction SilentlyContinue
        }
    }
}

$repoRoot = Get-RepoRoot
$manifestPath = Join-Path $repoRoot "Cargo.toml"

if (-not $Version) {
    $Version = Get-WorkspaceVersion -ManifestPath $manifestPath
}

$msixVersion = Convert-ToMsixVersion -Semver $Version
$X64ContentDir = Resolve-FullPath -Path $X64ContentDir -BasePath $repoRoot
$Arm64ContentDir = Resolve-FullPath -Path $Arm64ContentDir -BasePath $repoRoot
$OutputDir = Resolve-FullPath -Path $OutputDir -BasePath $repoRoot
$CliPath = Resolve-FullPath -Path $CliPath -BasePath $repoRoot
if ($CertificatePath) {
    $CertificatePath = Resolve-FullPath -Path $CertificatePath -BasePath $repoRoot
}

foreach ($path in @($X64ContentDir, $Arm64ContentDir, $CliPath, $CertificatePath) | Where-Object { $_ }) {
    if (-not (Test-Path $path)) {
        throw "Required input not found: $path"
    }
}

Write-StepLog "Resolved MSIX packaging inputs"
Write-StepLog "Workspace version: $Version"
Write-StepLog "MSIX version: $msixVersion"
Write-StepLog "x64 AppxContent dir: $X64ContentDir"
Write-StepLog "arm64 AppxContent dir: $Arm64ContentDir"
Write-StepLog "Output dir: $OutputDir"
Write-StepLog "msixbundle-cli path: $CliPath"
if ($CertificatePath) {
    Write-StepLog "Certificate path: $CertificatePath"
}
else {
    Write-StepLog "Certificate path not provided; temporary trust import will be skipped"
}

if (Test-Path $OutputDir) {
    Write-StepLog "Removing existing output directory $OutputDir"
    Remove-Item -Recurse -Force $OutputDir
}

New-Item -ItemType Directory -Force -Path $OutputDir | Out-Null
Write-StepLog "Prepared output directory $OutputDir"

$arguments = @(
    "--out-dir", $OutputDir,
    "--dir-x64", $X64ContentDir,
    "--dir-arm64", $Arm64ContentDir,
    "--timestamp-url", "",
    "--validate",
    "--force",
    "--verbose"
)

if ($PfxPath) {
    $PfxPath = Resolve-FullPath -Path $PfxPath -BasePath $repoRoot
    if (-not (Test-Path $PfxPath)) {
        throw "PFX file not found: $PfxPath"
    }

    if (-not $PfxPassword) {
        throw "PFX password is required when signing is enabled"
    }

    Write-StepLog "Signing is enabled for this MSIX packaging run"
    Write-StepLog "PFX path: $PfxPath"
}
else {
    Write-StepLog "No PFX path provided; MSIX packaging will run without signing"
}

$trustedCertificate = $null
$signingCertificate = $null
try {
    if ($CertificatePath) {
        Write-StepLog "Starting temporary certificate trust import"
        $trustedCertificate = Import-TemporaryTrustedCertificate -Path $CertificatePath
        $addedStoresMessage = if ($trustedCertificate.AddedStores.Count -gt 0) {
            $trustedCertificate.AddedStores -join ", "
        }
        else {
            "none"
        }
        Write-StepLog "Temporary certificate trust import complete; added stores: $addedStoresMessage"
    }
    else {
        Write-StepLog "Skipping temporary certificate trust import because no certificate path was provided"
    }

    if ($PfxPath) {
        Write-StepLog "Starting temporary signing certificate import"
        $signingCertificate = Import-TemporarySigningCertificate -Path $PfxPath -Password $PfxPassword
        $arguments += @(
            "--thumbprint", $signingCertificate.Thumbprint,
            "--cert-store", $signingCertificate.CertStore,
            "--sign-each",
            "--verify"
        )
        if ($signingCertificate.UseMachineStore) {
            $arguments += @("--machine-store")
        }
        Write-StepLog "Temporary signing certificate import complete; signing will use certificate thumbprint from cert store"
    }

    Write-StepLog "Invoking msixbundle-cli"
    $cliStopwatch = [System.Diagnostics.Stopwatch]::StartNew()
    & $CliPath @arguments
    $cliExitCode = $LASTEXITCODE
    $cliStopwatch.Stop()
    Write-StepLog ("msixbundle-cli finished in {0:N1}s with exit code {1}" -f $cliStopwatch.Elapsed.TotalSeconds, $cliExitCode)
    if ($cliExitCode -ne 0) {
        throw "msixbundle-cli failed with exit code $cliExitCode"
    }

    Write-StepLog "Locating generated MSIX/MSIXBUNDLE outputs"
    $generatedX64 = Get-ChildItem -Path $OutputDir -Filter "*_${msixVersion}_x64.msix" | Select-Object -First 1
    $generatedArm64 = Get-ChildItem -Path $OutputDir -Filter "*_${msixVersion}_arm64.msix" | Select-Object -First 1
    $generatedBundle = Get-ChildItem -Path $OutputDir -Filter "*_${msixVersion}.msixbundle" | Select-Object -First 1

    if (-not $generatedX64) {
        throw "Failed to locate generated x64 MSIX package in $OutputDir"
    }
    if (-not $generatedArm64) {
        throw "Failed to locate generated arm64 MSIX package in $OutputDir"
    }
    if (-not $generatedBundle) {
        throw "Failed to locate generated MSIX bundle in $OutputDir"
    }

    $finalX64 = Join-Path $OutputDir "$PackageBase-v$Version-x86_64-pc-windows-msvc.msix"
    $finalArm64 = Join-Path $OutputDir "$PackageBase-v$Version-aarch64-pc-windows-msvc.msix"
    $finalBundle = Join-Path $OutputDir "$PackageBase-v$Version-windows-msvc.msixbundle"

    Write-StepLog "Renaming generated outputs to release artifact names"
    Move-Item -Path $generatedX64.FullName -Destination $finalX64 -Force
    Move-Item -Path $generatedArm64.FullName -Destination $finalArm64 -Force
    Move-Item -Path $generatedBundle.FullName -Destination $finalBundle -Force

    Write-Host "MSIX package created: $finalX64"
    Write-Host "MSIX package created: $finalArm64"
    Write-Host "MSIX bundle created: $finalBundle"
}
finally {
    if ($signingCertificate -and $signingCertificate.AddedStores.Count -gt 0) {
        Write-StepLog "Cleaning up temporary signing certificate entries"
        Remove-TemporarySigningCertificate -Thumbprint $signingCertificate.Thumbprint -StorePaths $signingCertificate.AddedStores
    }

    if ($trustedCertificate -and $trustedCertificate.AddedStores.Count -gt 0) {
        Write-StepLog "Cleaning up temporary certificate trust entries"
        Remove-TemporaryTrustedCertificate -Thumbprint $trustedCertificate.Thumbprint -StorePaths $trustedCertificate.AddedStores
    }
}
