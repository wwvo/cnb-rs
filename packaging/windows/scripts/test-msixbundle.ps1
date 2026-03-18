[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [string]$BundlePath,

    [Parameter(Mandatory = $true)]
    [string]$CertificatePath,

    [string]$PackageName = "wwvo.cnb-rs",
    [string]$ExecutionAlias = "cnb-rs.exe",
    [string]$ExpectedVersion,
    [string]$ConfigDomain = "cnb.cool"
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

function Import-TestCertificate {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Path
    )

    $cert = New-Object System.Security.Cryptography.X509Certificates.X509Certificate2($Path)
    foreach ($storePath in @("Cert:\CurrentUser\Root", "Cert:\CurrentUser\TrustedPeople")) {
        Import-Certificate -FilePath $Path -CertStoreLocation $storePath | Out-Null
    }

    return $cert.Thumbprint
}

function Remove-TestCertificate {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Thumbprint
    )

    foreach ($storePath in @("Cert:\CurrentUser\Root", "Cert:\CurrentUser\TrustedPeople")) {
        Get-ChildItem $storePath | Where-Object Thumbprint -eq $Thumbprint | Remove-Item -Force -ErrorAction SilentlyContinue
    }
}

function Convert-ToCliVersion {
    param(
        [Parameter(Mandatory = $true)]
        [string]$PackageVersion
    )

    $match = [regex]::Match($PackageVersion, '^(?<major>\d+)\.(?<minor>\d+)\.(?<patch>\d+)')
    if (-not $match.Success) {
        throw "Unsupported package version format: $PackageVersion"
    }

    return "{0}.{1}.{2}" -f $match.Groups["major"].Value, $match.Groups["minor"].Value, $match.Groups["patch"].Value
}

function Invoke-AliasCommand {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Alias,

        [Parameter(Mandatory = $true)]
        [string[]]$Arguments
    )

    $argumentLiteral = [string]::Join(" ", ($Arguments | ForEach-Object { "'$_'" }))
    $command = "& '$Alias' $argumentLiteral"
    $windowsAppsPath = Join-Path $env:LOCALAPPDATA "Microsoft\WindowsApps"

    if (Test-Path $windowsAppsPath) {
        $pathEntries = $env:Path -split ';'
        if ($pathEntries -notcontains $windowsAppsPath) {
            $env:Path = "$windowsAppsPath;$env:Path"
        }
    }

    for ($attempt = 1; $attempt -le 10; $attempt++) {
        $output = & pwsh -NoLogo -NoProfile -Command $command 2>&1
        if ($LASTEXITCODE -eq 0) {
            return ($output -join [Environment]::NewLine)
        }

        Start-Sleep -Seconds 2
    }

    throw "Failed to invoke execution alias '$Alias' after package installation"
}

$BundlePath = [System.IO.Path]::GetFullPath($BundlePath)
$CertificatePath = [System.IO.Path]::GetFullPath($CertificatePath)

foreach ($path in @($BundlePath, $CertificatePath)) {
    if (-not (Test-Path $path)) {
        throw "Required input not found: $path"
    }
}

$configPath = Join-Path $HOME ".cnb\config.toml"
$backupPath = Join-Path $env:TEMP ("cnb-msixbundle-config-backup-{0}.toml" -f ([guid]::NewGuid().ToString("N")))
$configExisted = Test-Path $configPath
$thumbprint = $null
$package = $null

try {
    if ($configExisted) {
        Copy-Item $configPath $backupPath -Force
    }

    $thumbprint = Import-TestCertificate -Path $CertificatePath

    Add-AppxPackage -Path $BundlePath -ForceApplicationShutdown
    $package = Get-AppxPackage -Name $PackageName | Select-Object -First 1
    if (-not $package) {
        throw "Failed to locate installed package '$PackageName'"
    }

    if ($ExpectedVersion -and $package.Version.ToString() -ne $ExpectedVersion) {
        throw "Unexpected installed package version: $($package.Version) (expected $ExpectedVersion)"
    }

    $versionOutput = Invoke-AliasCommand -Alias $ExecutionAlias -Arguments @("--version")
    if ($ExpectedVersion) {
        $expectedCliVersion = Convert-ToCliVersion -PackageVersion $ExpectedVersion
        if ($versionOutput -notmatch [regex]::Escape($expectedCliVersion)) {
            throw "Execution alias output did not contain expected version '$expectedCliVersion'"
        }
    }

    $null = Invoke-AliasCommand -Alias $ExecutionAlias -Arguments @("config", "set", "domain", $ConfigDomain)
    if (-not (Test-Path $configPath)) {
        throw "Config file was not created by packaged CLI at $configPath"
    }

    $configContent = Get-Content $configPath -Raw
    if ($configContent -notmatch [regex]::Escape($ConfigDomain)) {
        throw "Config file did not contain expected domain '$ConfigDomain'"
    }
}
finally {
    if ($package) {
        Remove-AppxPackage -Package $package.PackageFullName -ErrorAction SilentlyContinue
    }

    if ($configExisted) {
        if (Test-Path $backupPath) {
            Copy-Item $backupPath $configPath -Force
        }
    }
    elseif (Test-Path $configPath) {
        Remove-Item $configPath -Force -ErrorAction SilentlyContinue
    }

    if (Test-Path $backupPath) {
        Remove-Item $backupPath -Force -ErrorAction SilentlyContinue
    }

    if ($thumbprint) {
        Remove-TestCertificate -Thumbprint $thumbprint
    }
}
