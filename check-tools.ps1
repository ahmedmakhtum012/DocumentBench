$ErrorActionPreference = 'Continue'
Write-Host 'Compiler and SDK checks'
Get-Command cl.exe,link.exe,winget.exe,dotnet.exe -ErrorAction SilentlyContinue | Format-Table Name,Source -AutoSize
$vswhere = Join-Path ${env:ProgramFiles(x86)} 'Microsoft Visual Studio\Installer\vswhere.exe'
if (Test-Path $vswhere) { & $vswhere -all -products * -format json }
foreach ($p in @(
  'C:\Program Files\Microsoft Visual Studio',
  'C:\Program Files (x86)\Microsoft Visual Studio',
  'C:\Program Files (x86)\Windows Kits\10\Lib',
  'C:\Program Files (x86)\Windows Kits\10\bin',
  'C:\BuildTools',
  "$env:LOCALAPPDATA\Microsoft\EdgeWebView"
)) {
  Write-Host "--- $p"
  if (Test-Path $p) { Get-ChildItem -LiteralPath $p -Name -ErrorAction SilentlyContinue | Select-Object -First 30 } else { Write-Host 'not present' }
}
Write-Host 'Nitro support libraries (filenames only)'
Get-ChildItem -LiteralPath 'C:\Program Files\Nitro\PDF Pro\CV' -Name -ErrorAction SilentlyContinue | Where-Object { $_ -match 'pdf|resource|en-US|UI|Core|ocr|tess|pdfium' } | Select-Object -First 80
