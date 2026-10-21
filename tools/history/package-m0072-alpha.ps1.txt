# Build inputs are reviewed separately; this script packages only a new local candidate.
[CmdletBinding()]
param(
    [Parameter(Mandatory)][string]$Binary,
    [Parameter(Mandatory)][string]$PreparedGame,
    [Parameter(Mandatory)][string]$Notices,
    [Parameter(Mandatory)][string]$RenderedExamples,
    [Parameter(Mandatory)][string]$RuntimeRepository,
    [Parameter(Mandatory)][string]$Output,
    [string]$Candidate = 'M006.1 local only',
    [string]$Showcase,
    [string]$BuildManifest
)
$ErrorActionPreference = 'Stop'
$editorRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$outputPath = [IO.Path]::GetFullPath($Output)
$zipPath = "$outputPath.zip"
if ((Test-Path -LiteralPath $outputPath) -or (Test-Path -LiteralPath $zipPath)) { throw 'Candidate or ZIP already exists; choose a new output. Existing candidates are never overwritten.' }
$inputs = @($Binary, $PreparedGame, $Notices, $RenderedExamples, $RuntimeRepository,
    (Join-Path $editorRoot 'samples/walker.ankh'), (Join-Path $editorRoot 'samples/walker.assets'),
    (Join-Path $editorRoot 'samples/minimal.ankh'), (Join-Path $editorRoot 'formats/tests/fixtures/tagged_rig.psd'))
foreach ($path in $inputs) { if (-not (Test-Path -LiteralPath $path)) { throw "Missing input: $path" } }
if (-not (Test-Path -LiteralPath (Join-Path $PreparedGame 'SNAPSHOT.json'))) { throw 'Prepare the pinned game first.' }
# Reject development debris or links in inputs. No discovery outside the explicit inputs.
foreach ($root in @($PreparedGame, $Notices, $RenderedExamples) + @($Showcase | Where-Object { $_ })) {
    foreach ($item in Get-ChildItem -LiteralPath $root -Recurse -Force) {
        if (($item.Attributes -band [IO.FileAttributes]::ReparsePoint) -or $item.Name -match '^(node_modules|\.git|\.env)$|autosave|backup' -or $item.Extension -in @('.pdb','.log','.tmp','.bak','.webm','.gif')) { throw "Unwanted packaging input: $($item.Name)" }
    }
}
$null = New-Item -ItemType Directory -Path $outputPath
$null = New-Item -ItemType Directory -Path (Join-Path $outputPath 'samples'), (Join-Path $outputPath 'docs')
Copy-Item -LiteralPath $Binary -Destination (Join-Path $outputPath 'Ankhimate.exe')
foreach ($name in @('walker.ankh','walker.assets','minimal.ankh')) { Copy-Item -LiteralPath (Join-Path $editorRoot "samples/$name") -Destination (Join-Path $outputPath 'samples') -Recurse }
Copy-Item -LiteralPath (Join-Path $editorRoot 'formats/tests/fixtures/tagged_rig.psd') -Destination (Join-Path $outputPath 'samples')
Copy-Item -LiteralPath $PreparedGame -Destination (Join-Path $outputPath 'game') -Recurse
Copy-Item -LiteralPath $Notices -Destination (Join-Path $outputPath 'licenses') -Recurse
Copy-Item -LiteralPath $RenderedExamples -Destination (Join-Path $outputPath 'exports') -Recurse
foreach ($name in @('alpha-scope.md','alpha-packaging.md','alpha-validation.md','alpha-bugs.md','alpha-final-check.md')) { Copy-Item -LiteralPath (Join-Path $editorRoot "docs/$name") -Destination (Join-Path $outputPath 'docs') }
if ($Showcase) { Copy-Item -LiteralPath $Showcase -Destination (Join-Path $outputPath 'showcase') -Recurse }
if ($BuildManifest) { Copy-Item -LiteralPath $BuildManifest -Destination (Join-Path $outputPath 'BUILD-PROVENANCE.json') }
Copy-Item -LiteralPath (Join-Path $RuntimeRepository 'docs/psd-to-phaser.md') -Destination (Join-Path $outputPath 'docs/psd-to-phaser.md')
$start = Get-Content -LiteralPath (Join-Path $editorRoot 'docs/alpha-start.md') -Raw
$start.Replace('](alpha-bugs.md)', '](docs/alpha-bugs.md)').Replace('](alpha-final-check.md)', '](docs/alpha-final-check.md)') | Set-Content -LiteralPath (Join-Path $outputPath 'README.md') -Encoding utf8
$versions = [ordered]@{
    candidate = $Candidate; editorCommit = (& git -C $editorRoot rev-parse HEAD).Trim()
    runtimePackagingCommit = (& git -C $RuntimeRepository rev-parse HEAD).Trim()
    runtimeCodeCommit = '8888e0e74ad642818c076c6446cca3d9358dd05f'
    runtimeSnapshot = 'm0061-8888e0e'; core = '0.1.1'; phaserAdapter = '0.1.0'; phaser = '3.90.0'; fflate = '0.8.3'
    runtimeFormat = 2; nodeTested = '24.18.0'; rustTested = '1.95.0'; target = 'x86_64-pc-windows-msvc'
    staticCRT = $true; signed = $false; cleanWindowsQualified = $false
    newCandidateGuiTested = $false
}
$versions | ConvertTo-Json | Set-Content -LiteralPath (Join-Path $outputPath 'VERSIONS.json') -Encoding utf8
$hashes = foreach ($file in Get-ChildItem -LiteralPath $outputPath -File -Recurse | Sort-Object FullName) {
    $relative = [IO.Path]::GetRelativePath($outputPath, $file.FullName).Replace('\','/')
    '{0}  {1}' -f (Get-FileHash -LiteralPath $file.FullName -Algorithm SHA256).Hash.ToLowerInvariant(), $relative
}
$hashes | Set-Content -LiteralPath (Join-Path $outputPath 'SHA256SUMS.txt') -Encoding utf8
# Failure leaves the new candidate for diagnosis, never replaces an older candidate.
Compress-Archive -LiteralPath $outputPath -DestinationPath $zipPath -CompressionLevel Optimal
$hash = (Get-FileHash -LiteralPath $zipPath -Algorithm SHA256).Hash.ToLowerInvariant()
"$hash  $([IO.Path]::GetFileName($zipPath))" | Set-Content -LiteralPath "$zipPath.sha256" -Encoding utf8
Write-Output "Local candidate prepared: $zipPath"
Write-Output "SHA-256: $hash"
