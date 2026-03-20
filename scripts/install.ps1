[CmdletBinding()]
param(
    [string]$Version,
    [ValidateSet("cnb", "github")]
    [string]$Source = $(if ($env:CNB_RS_INSTALL_SOURCE) { $env:CNB_RS_INSTALL_SOURCE } else { "cnb" }),
    [string]$InstallDir,
    [string]$RepoSlug = $(if ($env:CNB_RS_INSTALL_REPO_SLUG) { $env:CNB_RS_INSTALL_REPO_SLUG } else { "wwvo/cnb-rs/cnb-rs" }),
    [string]$CnbWebEndpoint = $(if ($env:CNB_RS_INSTALL_CNB_WEB_ENDPOINT) { $env:CNB_RS_INSTALL_CNB_WEB_ENDPOINT } else { "https://cnb.cool" }),
    [string]$GitHubRepoSlug = $(if ($env:CNB_RS_INSTALL_GITHUB_REPO_SLUG) { $env:CNB_RS_INSTALL_GITHUB_REPO_SLUG } else { "wwvo/cnb-rs" }),
    [string]$GitHubWebEndpoint = $(if ($env:CNB_RS_INSTALL_GITHUB_WEB_ENDPOINT) { $env:CNB_RS_INSTALL_GITHUB_WEB_ENDPOINT } else { "https://github.com" }),
    [switch]$NoPathUpdate
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$DefaultVersion = "v0.11.2"

function Write-StepLog {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Message
    )

    Write-Host ("[cnb-rs installer] {0}" -f $Message)
}

function Get-RequestParameters {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Uri
    )

    $params = @{
        Uri     = $Uri
        Headers = @{
            "User-Agent" = "cnb-rs-install.ps1"
            "Accept"     = "*/*"
        }
    }

    if ($PSVersionTable.PSVersion.Major -lt 6) {
        $params.UseBasicParsing = $true
    }

    return $params
}

function Invoke-DownloadFile {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Uri,

        [Parameter(Mandatory = $true)]
        [string]$Destination
    )

    $params = Get-RequestParameters -Uri $Uri
    $params.Headers["Accept"] = "application/octet-stream"
    $params.OutFile = $Destination

    Invoke-WebRequest @params | Out-Null
}

function Get-NormalizedVersion {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Value
    )

    if ([string]::IsNullOrWhiteSpace($Value)) {
        throw "Version must not be empty"
    }

    if ($Value.StartsWith("v")) {
        return $Value
    }

    return "v$Value"
}

function Get-ReleaseBaseUrl {
    param(
        [Parameter(Mandatory = $true)]
        [ValidateSet("cnb", "github")]
        [string]$Source,

        [Parameter(Mandatory = $true)]
        [string]$Version
    )

    switch ($Source.ToLowerInvariant()) {
        "cnb" {
            return "$CnbWebEndpoint/$RepoSlug/-/releases/download/$Version"
        }
        "github" {
            return "$GitHubWebEndpoint/$GitHubRepoSlug/releases/download/$Version"
        }
        default {
            throw "Unsupported download source: $Source"
        }
    }
}

function Get-TargetTriple {
    if (-not [System.Runtime.InteropServices.RuntimeInformation]::IsOSPlatform([System.Runtime.InteropServices.OSPlatform]::Windows)) {
        throw "install.ps1 can only be used on Windows; use install.sh on Linux or macOS"
    }

    switch ([System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture) {
        "X64" {
            return "x86_64-pc-windows-msvc"
        }
        "Arm64" {
            return "aarch64-pc-windows-msvc"
        }
        default {
            throw "Unsupported Windows architecture: $([System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture)"
        }
    }
}

function Get-DefaultInstallDir {
    if (-not [string]::IsNullOrWhiteSpace($env:LOCALAPPDATA)) {
        return (Join-Path $env:LOCALAPPDATA "Programs\cnb-rs\bin")
    }

    if (-not [string]::IsNullOrWhiteSpace($HOME)) {
        return (Join-Path $HOME "AppData\Local\Programs\cnb-rs\bin")
    }

    throw "Unable to determine a default install directory"
}

function Get-ExpectedChecksum {
    param(
        [Parameter(Mandatory = $true)]
        [string]$ChecksumFile,

        [Parameter(Mandatory = $true)]
        [string]$AssetName
    )

    foreach ($line in Get-Content -LiteralPath $ChecksumFile) {
        if ($line -match '^(?<hash>[0-9A-Fa-f]{64})\s+\*?(?<name>.+)$' -and $Matches["name"] -eq $AssetName) {
            return $Matches["hash"].ToLowerInvariant()
        }
    }

    throw "Failed to find a checksum entry for $AssetName"
}

function Add-UserPathEntry {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Entry
    )

    $trimmedEntry = $Entry.TrimEnd("\")
    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    $entries = @()

    if (-not [string]::IsNullOrWhiteSpace($userPath)) {
        $entries = $userPath -split ";" | Where-Object { -not [string]::IsNullOrWhiteSpace($_) }
    }

    $alreadyPresent = $entries | Where-Object { $_.TrimEnd("\") -ieq $trimmedEntry } | Select-Object -First 1
    if (-not $alreadyPresent) {
        $newEntries = @($entries + $Entry)
        [Environment]::SetEnvironmentVariable("Path", (($newEntries | Select-Object -Unique) -join ";"), "User")
        Write-StepLog "Added $Entry to the user PATH"
    }
    else {
        Write-StepLog "$Entry is already present in the user PATH"
    }

    $sessionEntries = $env:Path -split ";" | Where-Object { -not [string]::IsNullOrWhiteSpace($_) }
    $inSessionPath = $sessionEntries | Where-Object { $_.TrimEnd("\") -ieq $trimmedEntry } | Select-Object -First 1
    if (-not $inSessionPath) {
        $env:Path = "$Entry;$env:Path"
    }
}

$resolvedVersion = if ([string]::IsNullOrWhiteSpace($Version)) {
    Write-StepLog "Using bundled default release version $DefaultVersion"
    $DefaultVersion
}
else {
    Get-NormalizedVersion -Value $Version
}

$resolvedSource = $Source.ToLowerInvariant()
Write-StepLog "Using download source $resolvedSource"

$target = Get-TargetTriple
$assetName = "cnb-rs-$resolvedVersion-$target.zip"
$releaseBaseUrl = Get-ReleaseBaseUrl -Source $resolvedSource -Version $resolvedVersion

$tempRoot = Join-Path ([System.IO.Path]::GetTempPath()) ("cnb-rs-install-" + [System.Guid]::NewGuid().ToString("N"))
New-Item -ItemType Directory -Force -Path $tempRoot | Out-Null

try {
    $archivePath = Join-Path $tempRoot $assetName
    $checksumPath = Join-Path $tempRoot "sha256sum.txt"
    $extractDir = Join-Path $tempRoot "extract"

    Write-StepLog "Downloading $assetName"
    Invoke-DownloadFile -Uri "$releaseBaseUrl/$assetName" -Destination $archivePath

    Write-StepLog "Downloading sha256sum.txt"
    Invoke-DownloadFile -Uri "$releaseBaseUrl/sha256sum.txt" -Destination $checksumPath

    $expectedChecksum = Get-ExpectedChecksum -ChecksumFile $checksumPath -AssetName $assetName
    $actualChecksum = (Get-FileHash -Algorithm SHA256 -LiteralPath $archivePath).Hash.ToLowerInvariant()

    if ($actualChecksum -ne $expectedChecksum) {
        throw "SHA-256 checksum verification failed for $assetName"
    }

    Write-StepLog "Verified SHA-256 checksum for $assetName"

    Expand-Archive -LiteralPath $archivePath -DestinationPath $extractDir -Force

    $stagedBinary = [System.IO.Path]::Combine($extractDir, "cnb-rs-$resolvedVersion-$target", "cnb-rs.exe")
    if (-not (Test-Path -LiteralPath $stagedBinary)) {
        $candidate = Get-ChildItem -Path $extractDir -Filter "cnb-rs.exe" -Recurse | Select-Object -First 1
        if (-not $candidate) {
            throw "Failed to locate cnb-rs.exe in the extracted archive"
        }
        $stagedBinary = $candidate.FullName
    }

    $destinationDir = if ([string]::IsNullOrWhiteSpace($InstallDir)) {
        Get-DefaultInstallDir
    }
    else {
        $InstallDir
    }

    New-Item -ItemType Directory -Force -Path $destinationDir | Out-Null
    $resolvedInstallDir = (Resolve-Path -LiteralPath $destinationDir).Path
    $destinationPath = Join-Path $resolvedInstallDir "cnb-rs.exe"

    Copy-Item -LiteralPath $stagedBinary -Destination $destinationPath -Force

    if (-not $NoPathUpdate) {
        Add-UserPathEntry -Entry $resolvedInstallDir
    }

    $installedVersion = & $destinationPath --version 2>$null
    if ($LASTEXITCODE -eq 0 -and -not [string]::IsNullOrWhiteSpace($installedVersion)) {
        Write-StepLog "Installed $installedVersion to $destinationPath"
    }
    else {
        Write-StepLog "Installed cnb-rs to $destinationPath"
    }

    if ($NoPathUpdate) {
        Write-StepLog "PATH was not modified. Add $resolvedInstallDir to PATH if cnb-rs is not yet available in your shell."
    }
    else {
        Write-StepLog "Open a new terminal if cnb-rs is not yet available in your current shell."
    }
}
finally {
    Remove-Item -LiteralPath $tempRoot -Recurse -Force -ErrorAction SilentlyContinue
}
