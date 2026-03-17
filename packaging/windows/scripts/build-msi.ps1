[CmdletBinding()]
param(
    [string]$Target = "x86_64-pc-windows-msvc",
    [string]$Architecture = "x64",
    [string]$BinaryPath,
    [string]$LicensePath,
    [string]$Version,
    [string]$OutputDir = "target/windows-msi"
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

function Get-RepoRoot {
    $scriptDir = $PSScriptRoot
    return [System.IO.Path]::GetFullPath((Join-Path $scriptDir "..\..\.."))
}

function Get-CargoVersion {
    param(
        [Parameter(Mandatory = $true)]
        [string]$ManifestPath
    )

    $manifest = Get-Content $ManifestPath -Raw
    $match = [regex]::Match($manifest, '(?m)^\s*version\s*=\s*"(?<version>\d+\.\d+\.\d+)"\s*$')
    if (-not $match.Success) {
        throw "Failed to detect workspace version from $ManifestPath"
    }

    return $match.Groups["version"].Value
}

$repoRoot = Get-RepoRoot
$manifestPath = Join-Path $repoRoot "Cargo.toml"
$wixProjectPath = Join-Path $repoRoot "packaging\windows\wix\cnb-rs.wixproj"

if (-not $BinaryPath) {
    $BinaryPath = Join-Path $repoRoot "target\$Target\release\cnb-rs.exe"
}

if (-not $LicensePath) {
    $LicensePath = Join-Path $repoRoot "LICENSE"
}

if (-not $Version) {
    $Version = Get-CargoVersion -ManifestPath $manifestPath
}

$BinaryPath = [System.IO.Path]::GetFullPath($BinaryPath)
$LicensePath = [System.IO.Path]::GetFullPath($LicensePath)
$OutputDir = [System.IO.Path]::GetFullPath((Join-Path $repoRoot $OutputDir))
$BuildOutputDir = Join-Path $OutputDir "build"

if (-not (Test-Path $BinaryPath)) {
    throw "Binary not found: $BinaryPath"
}

if (-not (Test-Path $LicensePath)) {
    throw "License file not found: $LicensePath"
}

New-Item -ItemType Directory -Force -Path $OutputDir | Out-Null
New-Item -ItemType Directory -Force -Path $BuildOutputDir | Out-Null

$assetBase = "cnb-rs-v$Version-$Target"

dotnet build $wixProjectPath `
    -c Release `
    "-p:InstallerPlatform=$Architecture" `
    "-p:ProductVersion=$Version" `
    "-p:BinarySource=$BinaryPath" `
    "-p:LicenseSource=$LicensePath" `
    "-p:OutDir=$BuildOutputDir\"

if ($LASTEXITCODE -ne 0) {
    throw "WiX build failed with exit code $LASTEXITCODE"
}

$builtMsi = Join-Path $BuildOutputDir "cnb-rs.msi"
if (-not (Test-Path $builtMsi)) {
    throw "MSI output not found: $builtMsi"
}

$finalMsi = Join-Path $OutputDir "$assetBase.msi"
Copy-Item $builtMsi $finalMsi -Force
Write-Host "MSI package created: $finalMsi"
