[CmdletBinding()]
param(
    [string]$ExecutablePath,
    [string]$InboxPath,
    [ValidateRange(1, 30)]
    [int]$WaitSeconds = 15,
    [switch]$ResolveOnly
)

$ErrorActionPreference = "Stop"

function Get-DisplayIconPath([object]$Value) {
    if ($Value -isnot [string] -or [string]::IsNullOrWhiteSpace($Value)) { return $null }
    if ($Value -match '^"([^"]+)"') { return $matches[1] }
    return ($Value -split ',', 2)[0].Trim().Trim('"')
}

function Get-CandidatePaths {
    $paths = [System.Collections.Generic.List[string]]::new()
    if (-not [string]::IsNullOrWhiteSpace($ExecutablePath)) {
        $paths.Add($ExecutablePath)
        return $paths
    }

    foreach ($process in @(Get-Process -Name 'Memoryling' -ErrorAction SilentlyContinue)) {
        try {
            if (-not [string]::IsNullOrWhiteSpace($process.Path)) { $paths.Add($process.Path) }
        } catch {
            # A protected process path is not a launch candidate.
        }
    }

    $uninstallRoot = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\*'
    foreach ($entry in @(Get-ItemProperty -Path $uninstallRoot -ErrorAction SilentlyContinue |
        Where-Object { $_.DisplayName -eq 'Memoryling' })) {
        $iconPath = Get-DisplayIconPath $entry.DisplayIcon
        if (-not [string]::IsNullOrWhiteSpace($iconPath)) { $paths.Add($iconPath) }
        if (-not [string]::IsNullOrWhiteSpace($entry.InstallLocation)) {
            $paths.Add((Join-Path $entry.InstallLocation 'Memoryling.exe'))
        }
    }

    if (-not [string]::IsNullOrWhiteSpace($env:LOCALAPPDATA)) {
        $paths.Add((Join-Path $env:LOCALAPPDATA 'Memoryling\Memoryling.exe'))
        $paths.Add((Join-Path $env:LOCALAPPDATA 'Programs\Memoryling\Memoryling.exe'))
    }
    return $paths
}

function Resolve-MemorylingExecutable {
    $seen = [System.Collections.Generic.HashSet[string]]::new([System.StringComparer]::OrdinalIgnoreCase)
    foreach ($candidate in @(Get-CandidatePaths)) {
        try {
            $resolved = (Resolve-Path -LiteralPath $candidate -ErrorAction Stop).Path
            if (-not $seen.Add($resolved)) { continue }
            $item = Get-Item -LiteralPath $resolved -ErrorAction Stop
            if ($item.PSIsContainer -or $item.Name -ne 'Memoryling.exe') { continue }
            $version = [System.Diagnostics.FileVersionInfo]::GetVersionInfo($resolved)
            if ($version.ProductName -ne 'Memoryling') { continue }
            $numericVersion = [Version]::new(
                [Math]::Max(0, $version.FileMajorPart),
                [Math]::Max(0, $version.FileMinorPart),
                [Math]::Max(0, $version.FileBuildPart),
                [Math]::Max(0, $version.FilePrivatePart)
            )
            if ($numericVersion -lt [Version]'0.6.0.0') { continue }
            return $resolved
        } catch {
            # Missing, malformed, stale, or inaccessible candidates fail closed.
        }
    }
    throw 'Memoryling 0.6.0 or newer could not be found. Install or update the desktop app, then try again.'
}

$resolvedExecutable = Resolve-MemorylingExecutable
if ($ResolveOnly) {
    Write-Output 'Memoryling desktop app is ready to launch.'
    return
}

$expectedInbox = $null
if (-not [string]::IsNullOrWhiteSpace($InboxPath)) {
    if ([string]::IsNullOrWhiteSpace($env:LOCALAPPDATA)) {
        throw 'Memoryling local app data directory is unavailable.'
    }
    $expectedInbox = [IO.Path]::GetFullPath(
        (Join-Path $env:LOCALAPPDATA 'app.memoryling.desktop\agent-inbox\operation-v1.json')
    )
    if ([IO.Path]::GetFullPath($InboxPath) -ne $expectedInbox) {
        throw 'Memoryling refused to wait on an unexpected inbox path.'
    }
}

Start-Process -FilePath $resolvedExecutable

if ($null -eq $expectedInbox) {
    Write-Output 'Memoryling pet wake requested.'
    return
}

$deadline = [DateTimeOffset]::Now.AddSeconds($WaitSeconds)
while ([DateTimeOffset]::Now -lt $deadline) {
    if (-not (Test-Path -LiteralPath $expectedInbox)) {
        Write-Output 'Memoryling pet opened and the operation was applied.'
        return
    }
    Start-Sleep -Milliseconds 250
}

throw 'Memoryling was launched, but the operation was not confirmed within the local wait limit.'
