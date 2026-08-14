[CmdletBinding()]
param(
    [string]$Path,
    [string]$ExecutablePath,
    [ValidateRange(1, 30)]
    [int]$WaitSeconds = 15,
    [switch]$SkipAppReadyCheck,
    [switch]$SkipConfirmation,
    [switch]$CheckAppReadyOnly
)

$ErrorActionPreference = "Stop"
$AppNotReadyMessage = 'MEMORYLING_APP_NOT_READY: No compatible open Memoryling was found. Install it if needed, open the pet, then use the activation phrase again.'

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

function Test-CompatibleMemorylingExecutable([string]$Candidate) {
    try {
        $resolved = (Resolve-Path -LiteralPath $Candidate -ErrorAction Stop).Path
        $item = Get-Item -LiteralPath $resolved -ErrorAction Stop
        if ($item.PSIsContainer -or $item.Name -ne 'Memoryling.exe') { return $null }
        $version = [System.Diagnostics.FileVersionInfo]::GetVersionInfo($resolved)
        if ($version.ProductName -ne 'Memoryling') { return $null }
        $numericVersion = [Version]::new(
            [Math]::Max(0, $version.FileMajorPart),
            [Math]::Max(0, $version.FileMinorPart),
            [Math]::Max(0, $version.FileBuildPart),
            [Math]::Max(0, $version.FilePrivatePart)
        )
        if ($numericVersion -lt [Version]'0.7.0.0') { return $null }
        return $resolved
    } catch {
        return $null
    }
}

function Assert-MemorylingIsOpen {
    $expected = $null
    if (-not [string]::IsNullOrWhiteSpace($ExecutablePath)) {
        $expected = Test-CompatibleMemorylingExecutable $ExecutablePath
        if ($null -eq $expected) {
            throw $script:AppNotReadyMessage
        }
    }

    foreach ($process in @(Get-Process -Name 'Memoryling' -ErrorAction SilentlyContinue)) {
        try {
            if ([string]::IsNullOrWhiteSpace($process.Path)) { continue }
            $running = Test-CompatibleMemorylingExecutable $process.Path
            if ($null -eq $running) { continue }
            if ($null -eq $expected -or $running -eq $expected) { return }
        } catch {
            # Protected or stale process metadata is not proof that the App is ready.
        }
    }

    throw $script:AppNotReadyMessage
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

if ($CheckAppReadyOnly) {
    Assert-MemorylingIsOpen
    Write-Output 'Memoryling 0.7.0 or newer is open and ready for one bounded pet update.'
    return
}

if ([string]::IsNullOrWhiteSpace($Path)) {
    throw 'A protocol-v2 operation package path is required.'
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
$categories = @('opening', 'interaction', 'ambient', 'appearance')

if (-not (Test-ExactProperties $package @('schemaVersion', 'operationId', 'generatedAt', 'agent', 'sourceDigest', 'profile', 'appearancePlan', 'evidence', 'dialogues')) -or
    $package.schemaVersion -ne 2 -or -not (Test-Id $package.operationId) -or
    -not (Test-Timestamp $package.generatedAt) -or -not (Test-Hash $package.sourceDigest) -or
    -not (Test-ExactProperties $package.agent @('family')) -or $families -notcontains $package.agent.family -or
    -not (Test-ExactProperties $package.profile @('dominantActivity', 'journeyState') @('secondaryActivity')) -or
    $activities -notcontains $package.profile.dominantActivity -or
    ($null -ne $package.profile.secondaryActivity -and $activities -notcontains $package.profile.secondaryActivity) -or
    $journeys -notcontains $package.profile.journeyState -or
    -not (Test-ExactProperties $package.appearancePlan @('decision', 'qualification', 'evidenceIds') @('targetActivity', 'targetJourneyState')) -or
    @($package.evidence).Count -lt 1 -or @($package.evidence).Count -gt 12 -or
    @($package.dialogues).Count -ne 48) {
    Stop-InvalidPackage
}

$appearanceHasActivity = $package.appearancePlan.PSObject.Properties.Name -contains 'targetActivity'
$appearanceHasJourney = $package.appearancePlan.PSObject.Properties.Name -contains 'targetJourneyState'
$appearanceEvidenceCount = @($package.appearancePlan.evidenceIds).Count
switch ($package.appearancePlan.decision) {
    'hold' {
        if ($package.appearancePlan.qualification -ne 'insufficient-evidence' -or $appearanceHasActivity -or $appearanceHasJourney) { Stop-InvalidPackage }
    }
    'reset' {
        if ($package.appearancePlan.qualification -ne 'source-removed' -or $appearanceHasActivity -or $appearanceHasJourney) { Stop-InvalidPackage }
    }
    'change' {
        if (-not $appearanceHasActivity -or -not $appearanceHasJourney -or
            $activities -notcontains $package.appearancePlan.targetActivity -or
            $journeys -notcontains $package.appearancePlan.targetJourneyState -or
            @('consistent-signals', 'explicit-milestone') -notcontains $package.appearancePlan.qualification -or
            $appearanceEvidenceCount -lt 1 -or
            ($package.appearancePlan.qualification -eq 'consistent-signals' -and $appearanceEvidenceCount -lt 2)) {
            Stop-InvalidPackage
        }
    }
    default { Stop-InvalidPackage }
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

$appearanceEvidenceIds = [System.Collections.Generic.HashSet[string]]::new([System.StringComparer]::Ordinal)
foreach ($evidenceId in @($package.appearancePlan.evidenceIds)) {
    if (-not (Test-Id $evidenceId) -or -not $evidenceIds.Contains($evidenceId) -or -not $appearanceEvidenceIds.Add($evidenceId)) { Stop-InvalidPackage }
}

$dialogueIds = [System.Collections.Generic.HashSet[string]]::new([System.StringComparer]::Ordinal)
$seenTriggers = [System.Collections.Generic.HashSet[string]]::new([System.StringComparer]::Ordinal)
$categoryCounts = @{ opening = 0; interaction = 0; ambient = 0; appearance = 0 }
foreach ($dialogue in @($package.dialogues)) {
    if (-not (Test-ExactProperties $dialogue @('id', 'themeId', 'semanticGroup', 'category', 'text', 'trigger', 'priority', 'cooldownMinutes', 'maxUses', 'evidenceIds') @('notBefore', 'expiresAt')) -or
        -not (Test-Id $dialogue.id) -or -not $dialogueIds.Add($dialogue.id) -or
        -not (Test-Id $dialogue.themeId) -or -not (Test-Id $dialogue.semanticGroup) -or
        $categories -notcontains $dialogue.category -or
        -not (Test-ExactProperties $dialogue.text @('en', 'zhTw')) -or
        $dialogue.text.en -isnot [string] -or $dialogue.text.zhTw -isnot [string] -or
        [string]::IsNullOrWhiteSpace($dialogue.text.en) -or [string]::IsNullOrWhiteSpace($dialogue.text.zhTw) -or
        $dialogue.text.en.Length -gt 160 -or $dialogue.text.zhTw.Length -gt 160 -or
        $dialogue.text.en -match '[\r\n]' -or $dialogue.text.zhTw -match '[\r\n]' -or
        $triggers -notcontains $dialogue.trigger -or $dialogue.priority -lt 0 -or $dialogue.priority -gt 3 -or
        $dialogue.cooldownMinutes -lt 0 -or $dialogue.cooldownMinutes -gt 10080 -or
        $dialogue.maxUses -lt 1 -or $dialogue.maxUses -gt 20 -or @($dialogue.evidenceIds).Count -lt 1 -or
        ($null -ne $dialogue.notBefore -and -not (Test-Timestamp $dialogue.notBefore)) -or
        ($null -ne $dialogue.expiresAt -and -not (Test-Timestamp $dialogue.expiresAt))) {
        Stop-InvalidPackage
    }
    if (($dialogue.category -in @('opening', 'appearance') -and $dialogue.trigger -ne 'on-open') -or
        ($dialogue.category -eq 'interaction' -and $dialogue.trigger -ne 'on-interact') -or
        ($dialogue.category -eq 'ambient' -and $dialogue.trigger -ne 'ambient')) {
        Stop-InvalidPackage
    }
    $categoryCounts[$dialogue.category]++
    $null = $seenTriggers.Add($dialogue.trigger)
    $dialogueEvidenceIds = [System.Collections.Generic.HashSet[string]]::new([System.StringComparer]::Ordinal)
    foreach ($evidenceId in @($dialogue.evidenceIds)) {
        if (-not (Test-Id $evidenceId) -or -not $evidenceIds.Contains($evidenceId) -or -not $dialogueEvidenceIds.Add($evidenceId)) { Stop-InvalidPackage }
    }
}
if ($categoryCounts.opening -ne 8 -or $categoryCounts.interaction -ne 20 -or
    $categoryCounts.ambient -ne 16 -or $categoryCounts.appearance -ne 4 -or
    -not $seenTriggers.Contains('on-open') -or -not $seenTriggers.Contains('on-interact')) {
    Stop-InvalidPackage
}

if ([string]::IsNullOrWhiteSpace($env:LOCALAPPDATA)) {
    throw "Memoryling local app data directory is unavailable."
}

if (-not $SkipAppReadyCheck) {
    Assert-MemorylingIsOpen
}

$inboxDirectory = Join-Path $env:LOCALAPPDATA 'app.memoryling.desktop\agent-inbox'
$targetPath = Join-Path $inboxDirectory 'operation-v2.json'
$temporaryPath = Join-Path $inboxDirectory ('.operation-v2-' + [guid]::NewGuid().ToString('N') + '.tmp')
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

if (-not $SkipConfirmation) {
    $deadline = [DateTimeOffset]::Now.AddSeconds($WaitSeconds)
    while ([DateTimeOffset]::Now -lt $deadline) {
        if (-not (Test-Path -LiteralPath $targetPath)) {
            Write-Output ("Memoryling operation {0} was applied with {1} dialogue cards." -f $package.operationId, @($package.dialogues).Count)
            return
        }
        Start-Sleep -Milliseconds 250
    }
    if (Test-Path -LiteralPath $targetPath) {
        Remove-Item -LiteralPath $targetPath -Force
    }
    throw 'Memoryling is open, but the operation was not confirmed within the local wait limit. The unconfirmed inbox item was removed.'
}

Write-Output ("Submitted Memoryling operation {0} with {1} dialogue cards." -f $package.operationId, @($package.dialogues).Count)
