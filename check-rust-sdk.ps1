$ErrorActionPreference='Stop'
Write-Host 'SDK library and include locations'
Get-ChildItem -LiteralPath '.xwin-cache\unpack' -Directory -Recurse -ErrorAction SilentlyContinue | Where-Object { $_.Name -match 'x86_64|ucrt|um|shared|include' } | Select-Object -First 45 -ExpandProperty FullName
Get-ChildItem -LiteralPath '.tools\xwin\sdk\lib' -Recurse -Directory -ErrorAction SilentlyContinue | Select-Object -First 20 -ExpandProperty FullName
Write-Host 'Rust target state'
& "$env:USERPROFILE\.cargo\bin\rustup.exe" target list --installed
