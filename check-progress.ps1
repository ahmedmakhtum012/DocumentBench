$ErrorActionPreference='Continue'
Write-Host 'SDK download cache'
Get-ChildItem -LiteralPath '.xwin-cache' -Name -ErrorAction SilentlyContinue
Get-ChildItem -LiteralPath '.tools\xwin' -Depth 3 -Name -ErrorAction SilentlyContinue | Select-Object -First 70
Write-Host 'Build tool versions'
& "$env:USERPROFILE\.cargo\bin\rustc.exe" --version
& '.\.tools\node-v22.18.0-win-x64\node.exe' --version
