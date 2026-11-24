$ErrorActionPreference='Stop'
Add-Type -AssemblyName System.Drawing
$bmp = New-Object System.Drawing.Bitmap 256,256
$g=[System.Drawing.Graphics]::FromImage($bmp)
$g.SmoothingMode='AntiAlias'
$g.Clear([System.Drawing.Color]::FromArgb(68,38,143))
$white=[System.Drawing.Brushes]::White
$points=@([System.Drawing.Point]::new(62,42),[System.Drawing.Point]::new(144,42),[System.Drawing.Point]::new(194,92),[System.Drawing.Point]::new(194,214),[System.Drawing.Point]::new(62,214))
$g.FillPolygon($white,$points)
$purple=[System.Drawing.SolidBrush]::new([System.Drawing.Color]::FromArgb(68,38,143))
$light=[System.Drawing.SolidBrush]::new([System.Drawing.Color]::FromArgb(205,185,250))
$g.FillPolygon($light,@([System.Drawing.Point]::new(144,42),[System.Drawing.Point]::new(144,92),[System.Drawing.Point]::new(194,92)))
$font=[System.Drawing.Font]::new('Arial',78,[System.Drawing.FontStyle]::Bold)
$g.DrawString('N',$font,$purple,68,91)
$g.Flush()
$path=Join-Path $PSScriptRoot 'src-tauri\icons\app-icon.png'
$bmp.Save($path,[System.Drawing.Imaging.ImageFormat]::Png)
$g.Dispose();$bmp.Dispose()
