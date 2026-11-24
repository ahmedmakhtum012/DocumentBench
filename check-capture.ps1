Get-CimInstance Win32_Process | Where-Object { $_.Name -eq 'nitrocraft.exe' } | Select-Object ProcessId,CommandLine
Get-ChildItem -LiteralPath (Join-Path $PSScriptRoot 'checks') -Filter 'now.png*' -ErrorAction SilentlyContinue | Select-Object Name,Length
