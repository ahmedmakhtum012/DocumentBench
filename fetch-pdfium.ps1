$ErrorActionPreference = 'Stop'
$root = Join-Path $PSScriptRoot 'src-tauri\resources'
New-Item -ItemType Directory -Force -Path $root | Out-Null
$zip = Join-Path $PSScriptRoot '.tools\pdfium.zip'
New-Item -ItemType Directory -Force -Path (Split-Path $zip) | Out-Null
& curl.exe -fL --retry 2 'https://github.com/bblanchon/pdfium-binaries/releases/latest/download/pdfium-win-x64.tgz' -o (Join-Path $PSScriptRoot '.tools\pdfium.tgz')
if ($LASTEXITCODE -ne 0) { throw 'PDFium download failed' }
& tar.exe -xzf (Join-Path $PSScriptRoot '.tools\pdfium.tgz') -C (Join-Path $PSScriptRoot '.tools')
if ($LASTEXITCODE -ne 0) { throw 'PDFium extraction failed' }
Copy-Item (Join-Path $PSScriptRoot '.tools\bin\pdfium.dll') (Join-Path $root 'pdfium.dll') -Force
Copy-Item (Join-Path $PSScriptRoot '.tools\LICENSE') (Join-Path $root 'PDFium-LICENSE.txt') -Force -ErrorAction SilentlyContinue
Write-Host 'PDFium is ready.'
