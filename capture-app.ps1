$ErrorActionPreference='Stop'
& (Join-Path $PSScriptRoot 'target\debug\nitrocraft.exe') --screenshot (Join-Path $PSScriptRoot 'checks\now.png')
if($LASTEXITCODE -ne 0){throw "Screenshot capture failed with exit code $LASTEXITCODE"}
