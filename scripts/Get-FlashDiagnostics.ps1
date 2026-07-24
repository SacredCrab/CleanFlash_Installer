[CmdletBinding()]
param(
    [string]$OutputPath = ""
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version 2.0

if ([string]::IsNullOrWhiteSpace($OutputPath)) {
    $desktop = [Environment]::GetFolderPath([Environment+SpecialFolder]::DesktopDirectory)
    $OutputPath = Join-Path $desktop ("Flash_Diagnostics_{0}.txt" -f (Get-Date -Format "yyyyMMdd_HHmmss"))
}

$lines = New-Object System.Collections.Generic.List[string]
function Add-Line {
    param([string]$Text = "")
    $lines.Add($Text)
}

Add-Line "Clean Flash diagnostics"
Add-Line ("Generated: {0}" -f (Get-Date -Format "yyyy-MM-dd HH:mm:ss zzz"))
Add-Line ("PowerShell process: {0}-bit" -f ([IntPtr]::Size * 8))
Add-Line ("Operating system: {0}" -f [Environment]::OSVersion.VersionString)
Add-Line ("64-bit operating system: {0}" -f [Environment]::Is64BitOperatingSystem)
Add-Line ""

$activeXClsid = "{D27CDB6E-AE6D-11CF-96B8-444553540000}"
$views = @([Microsoft.Win32.RegistryView]::Registry32)
if ([Environment]::Is64BitOperatingSystem) {
    $views += [Microsoft.Win32.RegistryView]::Registry64
}

$paths = New-Object System.Collections.Generic.List[string]
Add-Line "Registered ActiveX OCX"
foreach ($view in $views) {
    $baseKey = [Microsoft.Win32.RegistryKey]::OpenBaseKey([Microsoft.Win32.RegistryHive]::ClassesRoot, $view)
    try {
        $key = $baseKey.OpenSubKey("CLSID\$activeXClsid\InprocServer32")
        try {
            $registeredPath = $null
            if ($null -ne $key) {
                $registeredPath = [string]$key.GetValue($null)
            }

            Add-Line ("  {0}: {1}" -f $view, $(if ([string]::IsNullOrWhiteSpace($registeredPath)) { "<not registered>" } else { $registeredPath }))
            if (-not [string]::IsNullOrWhiteSpace($registeredPath) -and -not $paths.Contains($registeredPath)) {
                $paths.Add($registeredPath)
            }
        } finally {
            if ($null -ne $key) {
                $key.Dispose()
            }
        }
    } finally {
        $baseKey.Dispose()
    }
}

$standardDirectories = @(
    (Join-Path $env:WINDIR "SysWOW64\Macromed\Flash"),
    (Join-Path $env:WINDIR "System32\Macromed\Flash")
)
foreach ($directory in $standardDirectories) {
    if (Test-Path -LiteralPath $directory -PathType Container) {
        Get-ChildItem -LiteralPath $directory -Filter "Flash*_*.ocx" -File -ErrorAction SilentlyContinue | ForEach-Object {
            if (-not $paths.Contains($_.FullName)) {
                $paths.Add($_.FullName)
            }
        }
    }
}

$knownHashes = @{
    "11206f2555bb41de8a254a8820a05106d4ae17848d9e57cdace973223a3612e5" = "34.0.0.376 Debug 32-bit ActiveX (known problematic case)"
    "4e1caa0b6805e5d4e4f737c4389b8e94e92ca49e7d7aef17c3773a9d744dcc42" = "34.0.0.376 Release 32-bit ActiveX"
}

Add-Line ""
Add-Line "OCX files"
foreach ($path in $paths) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        Add-Line ("  Missing: {0}" -f $path)
        continue
    }

    $item = Get-Item -LiteralPath $path
    $hash = (Get-FileHash -Algorithm SHA256 -LiteralPath $path).Hash.ToLowerInvariant()
    $classification = "Unknown build"
    if ($knownHashes.ContainsKey($hash)) {
        $classification = $knownHashes[$hash]
    }

    Add-Line ("  Path: {0}" -f $path)
    Add-Line ("  Size: {0}" -f $item.Length)
    Add-Line ("  File version: {0}" -f $item.VersionInfo.FileVersion)
    Add-Line ("  SHA-256: {0}" -f $hash)
    Add-Line ("  Classification: {0}" -f $classification)
    Add-Line ""
}

Add-Line "Interpretation"
Add-Line "  If the registered 32-bit OCX hash is the known Debug hash, install the ReleaseOnly package as administrator."
Add-Line "  If the registered OCX is already the Release hash, investigate the host program, SWF content, or a custom plugin copy."
Add-Line ""
Add-Line "Privacy"
Add-Line "  Review this report and remove personal paths or unrelated information before posting it publicly."

$parent = Split-Path -Parent $OutputPath
if (-not [string]::IsNullOrWhiteSpace($parent)) {
    New-Item -ItemType Directory -Path $parent -Force | Out-Null
}
$lines | Set-Content -LiteralPath $OutputPath -Encoding UTF8

Write-Host "Diagnostic report written to:"
Write-Host $OutputPath
