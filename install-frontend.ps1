$ErrorActionPreference = 'Stop'
$node = Join-Path $PSScriptRoot '.tools\node-v22.18.0-win-x64'
Write-Host "Node home: $node"
Get-ChildItem -LiteralPath $node -Name | Select-Object -First 15
$env:Path = "$node;$env:USERPROFILE\.cargo\bin;$env:Path"
& (Join-Path $node 'npm.cmd') install
if ($LASTEXITCODE -ne 0) { throw 'npm install failed' }
