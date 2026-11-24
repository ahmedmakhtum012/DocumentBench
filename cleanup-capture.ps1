$folder=Join-Path $PSScriptRoot 'checks'
Get-ChildItem -LiteralPath $folder -Filter 'now.png.*.log' -ErrorAction SilentlyContinue | Remove-Item -Force
