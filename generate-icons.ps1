$ErrorActionPreference='Stop'
$env:Path="$PSScriptRoot\.tools\node-v22.18.0-win-x64;$env:Path"
& (Join-Path $PSScriptRoot '.tools\node-v22.18.0-win-x64\npx.cmd') tauri icon (Join-Path $PSScriptRoot 'src-tauri\icons\app-icon.png')
if($LASTEXITCODE -ne 0){throw 'Icon generation failed'}
