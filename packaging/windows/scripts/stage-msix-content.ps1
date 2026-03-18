[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [string]$X64BinaryPath,

    [Parameter(Mandatory = $true)]
    [string]$Arm64BinaryPath,

    [string]$OutputDir = "target/windows-msix/appxcontent",
    [string]$IdentityName = "wwvo.cnb-rs",
    [string]$Publisher = "CN=cnb-rs CI",
    [string]$PublisherDisplayName = "wwvo",
    [string]$DisplayName = "cnb-rs",
    [string]$Description = "Unofficial CNB command-line tool built with Rust",
    [string]$ExecutionAlias = "cnb-rs.exe",
    [string]$Version,
    [string]$MinVersion = "10.0.17763.0",
    [string]$MaxVersionTested = "10.0.26100.0"
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

function Render-Manifest {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Template,

        [Parameter(Mandatory = $true)]
        [hashtable]$Values
    )

    $content = $Template
    foreach ($entry in $Values.GetEnumerator()) {
        $content = $content.Replace($entry.Key, $entry.Value)
    }

    return $content
}

$repoRoot = Get-RepoRoot
$manifestPath = Join-Path $repoRoot "Cargo.toml"
$templatePath = Join-Path $repoRoot "packaging\windows\msix\AppxManifest.xml.template"
$assetsDir = Join-Path $repoRoot "packaging\windows\msix\Assets"
$readmePath = Join-Path $repoRoot "README.md"
$licensePath = Join-Path $repoRoot "LICENSE"

if (-not $Version) {
    $Version = Convert-ToMsixVersion -Semver (Get-WorkspaceVersion -ManifestPath $manifestPath)
}

$template = Get-Content $templatePath -Raw
$X64BinaryPath = Resolve-FullPath -Path $X64BinaryPath -BasePath $repoRoot
$Arm64BinaryPath = Resolve-FullPath -Path $Arm64BinaryPath -BasePath $repoRoot
$OutputDir = Resolve-FullPath -Path $OutputDir -BasePath $repoRoot

foreach ($path in @($X64BinaryPath, $Arm64BinaryPath, $templatePath, $assetsDir, $readmePath, $licensePath)) {
    if (-not (Test-Path $path)) {
        throw "Required input not found: $path"
    }
}

if (Test-Path $OutputDir) {
    Remove-Item -Recurse -Force $OutputDir
}

$variants = @(
    @{
        Architecture = "x64"
        BinaryPath = $X64BinaryPath
    },
    @{
        Architecture = "arm64"
        BinaryPath = $Arm64BinaryPath
    }
)

foreach ($variant in $variants) {
    $appxContentDir = Join-Path $OutputDir $variant.Architecture
    $assetsOutputDir = Join-Path $appxContentDir "Assets"
    New-Item -ItemType Directory -Force -Path $assetsOutputDir | Out-Null

    Copy-Item $variant.BinaryPath (Join-Path $appxContentDir "cnb-rs.exe") -Force
    Copy-Item $readmePath (Join-Path $appxContentDir "README.md") -Force
    Copy-Item $licensePath (Join-Path $appxContentDir "LICENSE") -Force
    Copy-Item (Join-Path $assetsDir "*") $assetsOutputDir -Force

    $rendered = Render-Manifest -Template $template -Values @{
        "__IDENTITY_NAME__" = $IdentityName
        "__PUBLISHER__" = $Publisher
        "__VERSION__" = $Version
        "__PROCESSOR_ARCHITECTURE__" = $variant.Architecture
        "__DISPLAY_NAME__" = $DisplayName
        "__PUBLISHER_DISPLAY_NAME__" = $PublisherDisplayName
        "__DESCRIPTION__" = $Description
        "__EXECUTION_ALIAS__" = $ExecutionAlias
        "__MIN_VERSION__" = $MinVersion
        "__MAX_VERSION_TESTED__" = $MaxVersionTested
    }

    Set-Content -Path (Join-Path $appxContentDir "AppxManifest.xml") -Value $rendered -Encoding UTF8NoBOM
    Write-Host "Prepared AppxContent for $($variant.Architecture): $appxContentDir"
}
