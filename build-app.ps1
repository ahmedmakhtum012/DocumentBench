$ErrorActionPreference='Stop'
& (Join-Path $PSScriptRoot 'build-app.cmd')
if($LASTEXITCODE -ne 0){throw 'Application compilation failed'}
