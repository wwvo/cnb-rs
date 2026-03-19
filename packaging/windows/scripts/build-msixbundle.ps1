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

function Import-TemporaryTrustedCertificate {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Path
    )

    $cert = New-Object System.Security.Cryptography.X509Certificates.X509Certificate2($Path)
    $addedStores = @()
    foreach ($storePath in @(
        "Cert:\CurrentUser\Root",
        "Cert:\CurrentUser\TrustedPeople",
        "Cert:\LocalMachine\Root",
        "Cert:\LocalMachine\TrustedPeople"
    )) {
        $existing = Get-ChildItem $storePath | Where-Object Thumbprint -eq $cert.Thumbprint | Select-Object -First 1
        if (-not $existing) {
            Import-Certificate -FilePath $Path -CertStoreLocation $storePath | Out-Null
            $addedStores += $storePath
        }
    }

    return [pscustomobject]@{
        Thumbprint = $cert.Thumbprint
        AddedStores = $addedStores
    }
}

function Remove-TemporaryTrustedCertificate {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Thumbprint,

        [string[]]$StorePaths = @(
            "Cert:\CurrentUser\Root",
            "Cert:\CurrentUser\TrustedPeople",
            "Cert:\LocalMachine\Root",
            "Cert:\LocalMachine\TrustedPeople"
        )
    )

    foreach ($storePath in $StorePaths) {
        Get-ChildItem $storePath | Where-Object Thumbprint -eq $Thumbprint | Remove-Item -Force -ErrorAction SilentlyContinue
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

if (Test-Path $OutputDir) {
    Remove-Item -Recurse -Force $OutputDir
}

New-Item -ItemType Directory -Force -Path $OutputDir | Out-Null

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

    $arguments += @("--pfx", $PfxPath, "--sign-each", "--verify")
    if ($PfxPassword) {
        $arguments += @("--pfx-password", $PfxPassword)
    }
}

$trustedCertificate = $null
try {
    if ($CertificatePath) {
        $trustedCertificate = Import-TemporaryTrustedCertificate -Path $CertificatePath
    }

    & $CliPath @arguments
    if ($LASTEXITCODE -ne 0) {
        throw "msixbundle-cli failed with exit code $LASTEXITCODE"
    }

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

    Move-Item -Path $generatedX64.FullName -Destination $finalX64 -Force
    Move-Item -Path $generatedArm64.FullName -Destination $finalArm64 -Force
    Move-Item -Path $generatedBundle.FullName -Destination $finalBundle -Force

    Write-Host "MSIX package created: $finalX64"
    Write-Host "MSIX package created: $finalArm64"
    Write-Host "MSIX bundle created: $finalBundle"
}
finally {
    if ($trustedCertificate -and $trustedCertificate.AddedStores.Count -gt 0) {
        Remove-TemporaryTrustedCertificate -Thumbprint $trustedCertificate.Thumbprint -StorePaths $trustedCertificate.AddedStores
    }
}
