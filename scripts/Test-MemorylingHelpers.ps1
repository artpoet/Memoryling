[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'
$testRoot = Join-Path ([IO.Path]::GetTempPath()) ('memoryling-helper-test-' + [guid]::NewGuid().ToString('N'))
$originalLocalAppData = $env:LOCALAPPDATA

try {
    New-Item -ItemType Directory -Path $testRoot | Out-Null
    $env:LOCALAPPDATA = $testRoot
    $inbox = Join-Path $testRoot 'app.memoryling.desktop\agent-inbox\operation-v1.json'
    $wrongExecutable = Join-Path $env:WINDIR 'System32\notepad.exe'
    if (-not (Test-Path -LiteralPath $wrongExecutable -PathType Leaf)) {
        throw 'The helper test could not find its non-Memoryling control executable.'
    }

    $failedBeforeWrite = $false
    try {
        & (Join-Path $PSScriptRoot 'Submit-MemorylingOperation.ps1') `
            -Path (Join-Path $PSScriptRoot '..\examples\agent-operation-v1.synthetic.json') `
            -ExecutablePath $wrongExecutable
    } catch {
        $failedBeforeWrite = $_.Exception.Message -like 'Memoryling 0.6.0*' -and
            -not (Test-Path -LiteralPath $inbox)
    }
    if (-not $failedBeforeWrite) {
        throw 'An unavailable or mismatched App did not fail before inbox write.'
    }

    & (Join-Path $PSScriptRoot 'Submit-MemorylingOperation.ps1') `
        -Path (Join-Path $PSScriptRoot '..\examples\agent-operation-v1.synthetic.json') `
        -SkipLaunch | Out-Null
    if (-not (Test-Path -LiteralPath $inbox -PathType Leaf)) {
        throw 'The isolated no-launch submission did not create the exact inbox item.'
    }

    try {
        & (Join-Path $PSScriptRoot 'Start-Memoryling.ps1') `
            -ExecutablePath $wrongExecutable -ResolveOnly
        throw 'A differently named executable was accepted.'
    } catch {
        if ($_.Exception.Message -notlike 'Memoryling 0.6.0*') { throw }
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
