Get-Item -LiteralPath (Join-Path $PSScriptRoot 'src-tauri\resources\pdfium.dll') | Format-List FullName,Length
