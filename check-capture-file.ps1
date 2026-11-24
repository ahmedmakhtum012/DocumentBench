Add-Type -AssemblyName System.Drawing
$path = Join-Path $PSScriptRoot 'checks\resumed.png'
if (-not (Test-Path -LiteralPath $path)) { throw 'Screenshot was not created.' }
$image = [System.Drawing.Bitmap]::FromFile($path)
$colors = @{}
for ($y = 0; $y -lt $image.Height; $y += 30) {
    for ($x = 0; $x -lt $image.Width; $x += 30) {
        $pixel = $image.GetPixel($x, $y)
        $colors["$($pixel.R),$($pixel.G),$($pixel.B)"] = $true
    }
}
Write-Output "Screenshot: $path"
Write-Output "Dimensions: $($image.Width) x $($image.Height); bytes: $((Get-Item $path).Length); sampled colors: $($colors.Count)"
if ($image.Width -lt 1000 -or $image.Height -lt 650 -or $colors.Count -lt 10) { $image.Dispose(); throw 'Screenshot is blank or unexpectedly small.' }
$image.Dispose()
