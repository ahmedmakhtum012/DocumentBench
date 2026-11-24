Get-Process | Where-Object { $_.Name -match 'setup|vs_|install' } | Select-Object Name,Id,Path
Get-ChildItem -LiteralPath (Join-Path ${env:ProgramFiles(x86)} 'Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC') -ErrorAction SilentlyContinue | Select-Object FullName
