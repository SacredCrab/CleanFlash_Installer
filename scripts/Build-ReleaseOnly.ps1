[CmdletBinding()]
param(
    [string]$Version = "34.0.0.376",
    [string]$UpstreamTag = "v1.54",
    [string]$UpstreamAssetUrl = "https://github.com/darktohka/clean-flash-builds/releases/download/v1.54/ChineseFlash-Patched-Win-34.0.0.376.7z",
    [string]$UpstreamAssetSha256 = "19a8d1036110af024dc877ca96a9184835c3e944fb14ca8d509662f00bf1bd31",
    [string]$Configuration = "Release",
    [string]$OutputDirectory = ""
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version 2.0

$RepoRoot = Split-Path -Parent $PSScriptRoot
if ([string]::IsNullOrWhiteSpace($OutputDirectory)) {
    $OutputDirectory = Join-Path $RepoRoot "dist"
} elseif (-not [System.IO.Path]::IsPathRooted($OutputDirectory)) {
    $OutputDirectory = Join-Path $RepoRoot $OutputDirectory
}

$BuildRoot = Join-Path $RepoRoot ".build\release-only"
$DownloadDirectory = Join-Path $BuildRoot "download"
$ExtractDirectory = Join-Path $BuildRoot "upstream"
$PayloadDirectory = Join-Path $BuildRoot "payload"
$EmbeddedArchive = Join-Path $RepoRoot "CleanFlashInstaller\cleanflash.7z"
$UpstreamArchive = Join-Path $DownloadDirectory "ChineseFlash-Patched-Win-$Version.7z"

function Invoke-Checked {
    param(
        [Parameter(Mandatory = $true)][string]$Program,
        [Parameter(ValueFromRemainingArguments = $true)][string[]]$Arguments
    )

    & $Program @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "$Program exited with code $LASTEXITCODE"
    }
}

function Copy-RequiredFile {
    param(
        [Parameter(Mandatory = $true)][string]$Source,
        [Parameter(Mandatory = $true)][string]$DestinationDirectory
    )

    if (-not (Test-Path -LiteralPath $Source -PathType Leaf)) {
        throw "Required upstream file is missing: $Source"
    }

    New-Item -ItemType Directory -Path $DestinationDirectory -Force | Out-Null
    Copy-Item -LiteralPath $Source -Destination $DestinationDirectory -Force
}

function Get-SevenZipPath {
    $command = Get-Command "7z.exe" -ErrorAction SilentlyContinue
    if ($null -ne $command) {
        return $command.Source
    }

    $candidates = New-Object System.Collections.Generic.List[string]
    if (-not [string]::IsNullOrWhiteSpace($env:ProgramFiles)) {
        $candidates.Add((Join-Path $env:ProgramFiles "7-Zip\7z.exe"))
    }
    $programFilesX86 = [Environment]::GetEnvironmentVariable("ProgramFiles(x86)")
    if (-not [string]::IsNullOrWhiteSpace($programFilesX86)) {
        $candidates.Add((Join-Path $programFilesX86 "7-Zip\7z.exe"))
    }

    foreach ($candidate in $candidates) {
        if (-not [string]::IsNullOrWhiteSpace($candidate) -and (Test-Path -LiteralPath $candidate -PathType Leaf)) {
            return $candidate
        }
    }

    throw "7-Zip was not found. Install 7-Zip or add 7z.exe to PATH."
}

if (Test-Path -LiteralPath $BuildRoot) {
    Remove-Item -LiteralPath $BuildRoot -Recurse -Force
}
New-Item -ItemType Directory -Path $DownloadDirectory, $ExtractDirectory, $PayloadDirectory, $OutputDirectory -Force | Out-Null

$SevenZip = Get-SevenZipPath
$DotNet = (Get-Command "dotnet.exe" -ErrorAction Stop).Source

$installFormSource = Get-Content -LiteralPath (Join-Path $RepoRoot "CleanFlashInstaller\InstallForm.cs") -Raw
if ($installFormSource -match "Set(?:Flag|Conditionally)\s*\([^\)]*InstallFlags\.DEBUG") {
    throw "ReleaseOnly invariant failed: InstallForm.cs can set InstallFlags.DEBUG."
}

$installerSource = Get-Content -LiteralPath (Join-Path $RepoRoot "CleanFlashInstaller\Installer.cs") -Raw
if ($installerSource -notmatch 'filename\.IndexOf\("-debug",\s*StringComparison\.OrdinalIgnoreCase\)\s*>=\s*0') {
    throw "ReleaseOnly invariant failed: Installer.cs does not reject -debug payloads."
}

Write-Host "Downloading pinned upstream Flash package $UpstreamTag..."
Invoke-WebRequest -UseBasicParsing -Uri $UpstreamAssetUrl -OutFile $UpstreamArchive

$actualUpstreamHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $UpstreamArchive).Hash.ToLowerInvariant()
if ($actualUpstreamHash -ne $UpstreamAssetSha256.ToLowerInvariant()) {
    throw "Upstream SHA-256 mismatch. Expected $UpstreamAssetSha256, got $actualUpstreamHash."
}

Write-Host "Extracting upstream package..."
Invoke-Checked $SevenZip "x" $UpstreamArchive "-o$ExtractDirectory" "-y"

Write-Host "Building the uninstaller..."
Invoke-Checked $DotNet "build" (Join-Path $RepoRoot "CleanFlashUninstaller\CleanFlashUninstaller.csproj") "-c" $Configuration "--nologo" "--verbosity" "minimal"

$uninstaller = Get-ChildItem -Path (Join-Path $RepoRoot "CleanFlashUninstaller\bin\$Configuration") -Filter "CleanFlashUninstaller.exe" -File -Recurse |
    Select-Object -First 1
if ($null -eq $uninstaller) {
    throw "CleanFlashUninstaller.exe was not produced."
}

$versionPath = $Version.Replace(".", "_")

Copy-RequiredFile (Join-Path $ExtractDirectory "controlpanel\FlashPlayerApp.exe") (Join-Path $PayloadDirectory "controlpanel")
Copy-RequiredFile (Join-Path $ExtractDirectory "controlpanel\FlashPlayerCPLApp.cpl") (Join-Path $PayloadDirectory "controlpanel")

Copy-RequiredFile (Join-Path $ExtractDirectory "flash32\flashplayer.xpt") (Join-Path $PayloadDirectory "np32")
Copy-RequiredFile (Join-Path $ExtractDirectory "flash32\NPSWF32_$versionPath.dll") (Join-Path $PayloadDirectory "np32")
Copy-RequiredFile (Join-Path $ExtractDirectory "flash64\NPSWF64_$versionPath.dll") (Join-Path $PayloadDirectory "np64")

Copy-RequiredFile (Join-Path $ExtractDirectory "flash32\manifest.json") (Join-Path $PayloadDirectory "pp32")
Copy-RequiredFile (Join-Path $ExtractDirectory "flash32\pepflashplayer32_$versionPath.dll") (Join-Path $PayloadDirectory "pp32")
Copy-RequiredFile (Join-Path $ExtractDirectory "flash64\manifest.json") (Join-Path $PayloadDirectory "pp64")
Copy-RequiredFile (Join-Path $ExtractDirectory "flash64\pepflashplayer64_$versionPath.dll") (Join-Path $PayloadDirectory "pp64")

Copy-RequiredFile (Join-Path $ExtractDirectory "flash32\Flash32_$versionPath.ocx") (Join-Path $PayloadDirectory "ocx32")
Copy-RequiredFile (Join-Path $ExtractDirectory "flash32\win7-and-older\Flash32_$versionPath.ocx") (Join-Path $PayloadDirectory "ocx32-legacy")
Copy-RequiredFile (Join-Path $ExtractDirectory "flash64\Flash64_$versionPath.ocx") (Join-Path $PayloadDirectory "ocx64")
Copy-RequiredFile (Join-Path $ExtractDirectory "flash64\win7-and-older\Flash64_$versionPath.ocx") (Join-Path $PayloadDirectory "ocx64-legacy")

Copy-RequiredFile (Join-Path $ExtractDirectory "standalone-projector\flashplayer_sa.exe") (Join-Path $PayloadDirectory "standalone")

$uninstallerDirectory = Join-Path $PayloadDirectory "uninstaller"
New-Item -ItemType Directory -Path $uninstallerDirectory -Force | Out-Null
Copy-Item -LiteralPath $uninstaller.FullName -Destination (Join-Path $uninstallerDirectory "FlashUtil_Uninstall.exe") -Force

$debugPayloads = Get-ChildItem -Path $PayloadDirectory -File -Recurse |
    Where-Object { $_.Name -match "(?i)debug" -or $_.DirectoryName -match "(?i)debug" }
if ($null -ne $debugPayloads) {
    $firstDebugPayload = $debugPayloads | Select-Object -First 1
    throw "ReleaseOnly payload unexpectedly contains a Debug file: $($firstDebugPayload.FullName)"
}

$releaseOcx = Join-Path $PayloadDirectory "ocx32\Flash32_$versionPath.ocx"
$releaseOcxHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $releaseOcx).Hash.ToLowerInvariant()
$expectedReleaseOcxHash = "4e1caa0b6805e5d4e4f737c4389b8e94e92ca49e7d7aef17c3773a9d744dcc42"
if ($releaseOcxHash -ne $expectedReleaseOcxHash) {
    throw "32-bit Release OCX SHA-256 mismatch. Expected $expectedReleaseOcxHash, got $releaseOcxHash."
}

if (Test-Path -LiteralPath $EmbeddedArchive) {
    Remove-Item -LiteralPath $EmbeddedArchive -Force
}

Write-Host "Creating deterministic ReleaseOnly payload archive..."
Push-Location $PayloadDirectory
try {
    Invoke-Checked $SevenZip "a" "-t7z" $EmbeddedArchive ".\*" "-mx=9" "-m0=LZMA2:d=26" "-ms=on" "-mmt=off" "-mtm=off" "-mta=off" "-mtc=off"
} finally {
    Pop-Location
}
Invoke-Checked $SevenZip "t" $EmbeddedArchive

Write-Host "Building the installer..."
Invoke-Checked $DotNet "build" (Join-Path $RepoRoot "CleanFlashInstaller\CleanFlashInstaller.csproj") "-c" $Configuration "--nologo" "--verbosity" "minimal"

$builtInstaller = Get-ChildItem -Path (Join-Path $RepoRoot "CleanFlashInstaller\bin\$Configuration") -Filter "CleanFlashInstaller.exe" -File -Recurse |
    Select-Object -First 1
if ($null -eq $builtInstaller) {
    throw "CleanFlashInstaller.exe was not produced."
}

$finalInstaller = Join-Path $OutputDirectory "CleanFlash_${Version}_ReleaseOnly_Installer.exe"
Copy-Item -LiteralPath $builtInstaller.FullName -Destination $finalInstaller -Force

Write-Host "Verifying embedded archive..."
$assembly = [System.Reflection.Assembly]::LoadFile($finalInstaller)
$resourceStream = $assembly.GetManifestResourceStream("CleanFlashInstaller.cleanflash.7z")
if ($null -eq $resourceStream) {
    throw "The installer does not contain CleanFlashInstaller.cleanflash.7z."
}

$verificationArchive = Join-Path $BuildRoot "embedded-verification.7z"
$verificationOutput = [System.IO.File]::Create($verificationArchive)
try {
    $resourceStream.CopyTo($verificationOutput)
} finally {
    $verificationOutput.Dispose()
    $resourceStream.Dispose()
}
Invoke-Checked $SevenZip "t" $verificationArchive

$archiveListing = (& $SevenZip "l" "-slt" $verificationArchive | Out-String)
if ($LASTEXITCODE -ne 0) {
    throw "Unable to list the embedded archive."
}
if ($archiveListing -match "(?im)^Path = .*debug") {
    throw "The embedded archive contains a Debug path."
}

$installerHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $finalInstaller).Hash.ToLowerInvariant()
$checksumsPath = Join-Path $OutputDirectory "SHA256SUMS.txt"
"$installerHash  $(Split-Path -Leaf $finalInstaller)" | Set-Content -LiteralPath $checksumsPath -Encoding Ascii

Write-Host ""
Write-Host "Build completed:"
Write-Host "  Installer: $finalInstaller"
Write-Host "  SHA-256:   $installerHash"
Write-Host "  Checksums: $checksumsPath"
