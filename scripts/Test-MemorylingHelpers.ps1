[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'
$testRoot = Join-Path ([IO.Path]::GetTempPath()) ('memoryling-helper-test-' + [guid]::NewGuid().ToString('N'))
$originalLocalAppData = $env:LOCALAPPDATA

try {
    New-Item -ItemType Directory -Path $testRoot | Out-Null
    $env:LOCALAPPDATA = $testRoot
    $inbox = Join-Path $testRoot 'app.memoryling.desktop\agent-inbox\operation-v2.json'
    $wrongExecutable = Join-Path $env:WINDIR 'System32\notepad.exe'
    if (-not (Test-Path -LiteralPath $wrongExecutable -PathType Leaf)) {
        throw 'The helper test could not find its non-Memoryling control executable.'
    }

    $readinessReminderWasSafe = $false
    try {
        & (Join-Path $PSScriptRoot 'Submit-MemorylingOperation.ps1') `
            -CheckAppReadyOnly -ExecutablePath $wrongExecutable
    } catch {
        $message = $_.Exception.Message
        $readinessReminderWasSafe = $message -like 'MEMORYLING_APP_NOT_READY:*' -and
            $message -like '*Install it if needed*' -and
            $message -like '*activation phrase again*' -and
            -not (Test-Path -LiteralPath $inbox)
    }
    if (-not $readinessReminderWasSafe) {
        throw 'Readiness-only failure did not provide the safe install/open reminder before inbox write.'
    }

    $failedBeforeWrite = $false
    try {
        & (Join-Path $PSScriptRoot 'Submit-MemorylingOperation.ps1') `
            -Path (Join-Path $PSScriptRoot '..\examples\agent-operation-v2.synthetic.json') `
            -ExecutablePath $wrongExecutable
    } catch {
        $failedBeforeWrite = $_.Exception.Message -like 'MEMORYLING_APP_NOT_READY:*' -and
            -not (Test-Path -LiteralPath $inbox)
    }
    if (-not $failedBeforeWrite) {
        throw 'A closed or mismatched App did not fail before inbox write.'
    }

    $processCountBefore = @(Get-Process -Name 'Memoryling' -ErrorAction SilentlyContinue).Count
    & (Join-Path $PSScriptRoot 'Submit-MemorylingOperation.ps1') `
        -Path (Join-Path $PSScriptRoot '..\examples\agent-operation-v2.synthetic.json') `
        -SkipAppReadyCheck -SkipConfirmation | Out-Null
    if (-not (Test-Path -LiteralPath $inbox -PathType Leaf)) {
        throw 'The isolated submission did not create the exact inbox item.'
    }
    $processCountAfter = @(Get-Process -Name 'Memoryling' -ErrorAction SilentlyContinue).Count
    if ($processCountAfter -ne $processCountBefore) {
        throw 'Submitting an operation unexpectedly changed the Memoryling process count.'
    }

    $unconfirmedWasRemoved = $false
    try {
        & (Join-Path $PSScriptRoot 'Submit-MemorylingOperation.ps1') `
            -Path (Join-Path $PSScriptRoot '..\examples\agent-operation-v2.synthetic.json') `
            -SkipAppReadyCheck -WaitSeconds 1 | Out-Null
    } catch {
        $unconfirmedWasRemoved = $_.Exception.Message -like '*unconfirmed inbox item was removed*' -and
            -not (Test-Path -LiteralPath $inbox)
    }
    if (-not $unconfirmedWasRemoved) {
        throw 'An unconfirmed inbox item was not removed after the bounded wait.'
    }

    Write-Output 'Memoryling helper safety tests passed.'
} finally {
    $env:LOCALAPPDATA = $originalLocalAppData
    $resolvedTestRoot = [IO.Path]::GetFullPath($testRoot)
    $resolvedTempRoot = [IO.Path]::GetFullPath([IO.Path]::GetTempPath())
    if ($resolvedTestRoot.StartsWith($resolvedTempRoot, [StringComparison]::OrdinalIgnoreCase) -and
        (Test-Path -LiteralPath $resolvedTestRoot -PathType Container)) {
        [IO.Directory]::Delete($resolvedTestRoot, $true)
    }
}
