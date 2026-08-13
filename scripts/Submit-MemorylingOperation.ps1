[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [string]$Path,
    [string]$ExecutablePath,
    [switch]$SkipLaunch
)

$ErrorActionPreference = "Stop"

function Stop-InvalidPackage {
    throw "Memoryling operation validation failed. No package was submitted."
}

function Test-Id([object]$Value) {
    return $Value -is [string] -and $Value -match '^[a-z0-9][a-z0-9._-]{0,63}$'
}

function Test-Hash([object]$Value) {
    return $Value -is [string] -and $Value -match '^[a-f0-9]{64}$'
}

function Test-Timestamp([object]$Value) {
    if ($Value -isnot [string]) { return $false }
    $parsed = [DateTimeOffset]::MinValue
    return [DateTimeOffset]::TryParse($Value, [ref]$parsed)
}

function Test-ExactProperties([object]$Value, [string[]]$Required, [string[]]$Optional = @()) {
    if ($null -eq $Value) { return $false }
    $actual = @($Value.PSObject.Properties.Name)
    foreach ($name in $Required) {
        if ($actual -notcontains $name) { return $false }
    }
    foreach ($name in $actual) {
        if ($Required -notcontains $name -and $Optional -notcontains $name) { return $false }
    }
    return $true
}

$sourcePath = (Resolve-Path -LiteralPath $Path).Path
$item = Get-Item -LiteralPath $sourcePath
if (-not $item.PSIsContainer -and $item.Length -gt 0 -and $item.Length -le 65536) {
    $package = Get-Content -LiteralPath $sourcePath -Raw -Encoding UTF8 | ConvertFrom-Json
} else {
    Stop-InvalidPackage
}

$activities = @('building', 'research', 'design', 'planning', 'debugging', 'writing', 'coordination', 'shipping')
$journeys = @('steady', 'exploring', 'milestone', 'recovering')
$families = @('codex', 'claude', 'other')
$evidenceKinds = @('durable-memory', 'recent-work', 'repo-ssot', 'current-thread')
$triggers = @('on-open', 'on-interact', 'ambient')

if (-not (Test-ExactProperties $package @('schemaVersion', 'operationId', 'generatedAt', 'agent', 'sourceDigest', 'profile', 'evidence', 'dialogues')) -or
    $package.schemaVersion -ne 1 -or -not (Test-Id $package.operationId) -or
    -not (Test-Timestamp $package.generatedAt) -or -not (Test-Hash $package.sourceDigest) -or
    -not (Test-ExactProperties $package.agent @('family')) -or $families -notcontains $package.agent.family -or
    -not (Test-ExactProperties $package.profile @('dominantActivity', 'journeyState') @('secondaryActivity')) -or
    $activities -notcontains $package.profile.dominantActivity -or
    ($null -ne $package.profile.secondaryActivity -and $activities -notcontains $package.profile.secondaryActivity) -or
    $journeys -notcontains $package.profile.journeyState -or
    @($package.evidence).Count -lt 1 -or @($package.evidence).Count -gt 12 -or
    @($package.dialogues).Count -lt 3 -or @($package.dialogues).Count -gt 12) {
    Stop-InvalidPackage
}

$evidenceIds = [System.Collections.Generic.HashSet[string]]::new([System.StringComparer]::Ordinal)
foreach ($evidence in @($package.evidence)) {
    if (-not (Test-ExactProperties $evidence @('id', 'kind', 'referenceHash', 'observedAt')) -or
        -not (Test-Id $evidence.id) -or -not $evidenceIds.Add($evidence.id) -or
        $evidenceKinds -notcontains $evidence.kind -or -not (Test-Hash $evidence.referenceHash) -or
        -not (Test-Timestamp $evidence.observedAt)) {
        Stop-InvalidPackage
    }
}

$dialogueIds = [System.Collections.Generic.HashSet[string]]::new([System.StringComparer]::Ordinal)
$seenTriggers = [System.Collections.Generic.HashSet[string]]::new([System.StringComparer]::Ordinal)
foreach ($dialogue in @($package.dialogues)) {
    if (-not (Test-ExactProperties $dialogue @('id', 'text', 'trigger', 'priority', 'cooldownMinutes', 'maxUses', 'evidenceIds') @('notBefore', 'expiresAt')) -or
        -not (Test-Id $dialogue.id) -or -not $dialogueIds.Add($dialogue.id) -or
        -not (Test-ExactProperties $dialogue.text @('en', 'zhTw')) -or
        $dialogue.text.en -isnot [string] -or $dialogue.text.zhTw -isnot [string] -or
        [string]::IsNullOrWhiteSpace($dialogue.text.en) -or [string]::IsNullOrWhiteSpace($dialogue.text.zhTw) -or
        $dialogue.text.en.Length -gt 240 -or $dialogue.text.zhTw.Length -gt 240 -or
        $dialogue.text.en -match '[\r\n]' -or $dialogue.text.zhTw -match '[\r\n]' -or
        $triggers -notcontains $dialogue.trigger -or $dialogue.priority -lt 0 -or $dialogue.priority -gt 3 -or
        $dialogue.cooldownMinutes -lt 0 -or $dialogue.cooldownMinutes -gt 10080 -or
        $dialogue.maxUses -lt 1 -or $dialogue.maxUses -gt 20 -or @($dialogue.evidenceIds).Count -lt 1 -or
        ($null -ne $dialogue.notBefore -and -not (Test-Timestamp $dialogue.notBefore)) -or
        ($null -ne $dialogue.expiresAt -and -not (Test-Timestamp $dialogue.expiresAt))) {
        Stop-InvalidPackage
    }
    $null = $seenTriggers.Add($dialogue.trigger)
    $dialogueEvidenceIds = [System.Collections.Generic.HashSet[string]]::new([System.StringComparer]::Ordinal)
    foreach ($evidenceId in @($dialogue.evidenceIds)) {
        if (-not (Test-Id $evidenceId) -or -not $evidenceIds.Contains($evidenceId) -or -not $dialogueEvidenceIds.Add($evidenceId)) { Stop-InvalidPackage }
    }
}
if (-not $seenTriggers.Contains('on-open') -or -not $seenTriggers.Contains('on-interact')) {
    Stop-InvalidPackage
}

if ([string]::IsNullOrWhiteSpace($env:LOCALAPPDATA)) {
    throw "Memoryling local app data directory is unavailable."
}

$launcherPath = Join-Path $PSScriptRoot 'Start-Memoryling.ps1'
if (-not $SkipLaunch) {
    $resolveArguments = @{ ResolveOnly = $true }
    if (-not [string]::IsNullOrWhiteSpace($ExecutablePath)) {
        $resolveArguments.ExecutablePath = $ExecutablePath
    }
    & $launcherPath @resolveArguments | Out-Null
}

$inboxDirectory = Join-Path $env:LOCALAPPDATA 'app.memoryling.desktop\agent-inbox'
$targetPath = Join-Path $inboxDirectory 'operation-v1.json'
$temporaryPath = Join-Path $inboxDirectory ('.operation-v1-' + [guid]::NewGuid().ToString('N') + '.tmp')
New-Item -ItemType Directory -Path $inboxDirectory -Force | Out-Null
$utf8NoBom = New-Object System.Text.UTF8Encoding($false)
try {
    $serialized = $package | ConvertTo-Json -Depth 8 -Compress
    [System.IO.File]::WriteAllText($temporaryPath, $serialized, $utf8NoBom)
    Move-Item -LiteralPath $temporaryPath -Destination $targetPath -Force
} finally {
    if (Test-Path -LiteralPath $temporaryPath) {
        Remove-Item -LiteralPath $temporaryPath -Force
    }
}

if (-not $SkipLaunch) {
    $launchArguments = @{ InboxPath = $targetPath }
    if (-not [string]::IsNullOrWhiteSpace($ExecutablePath)) {
        $launchArguments.ExecutablePath = $ExecutablePath
    }
    & $launcherPath @launchArguments
}

Write-Output ("Submitted Memoryling operation {0} with {1} dialogue cards." -f $package.operationId, @($package.dialogues).Count)
