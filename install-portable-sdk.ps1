$ErrorActionPreference='Stop'
$root=Join-Path $PSScriptRoot '.tools'
New-Item -ItemType Directory -Force -Path $root | Out-Null
$env:Path="$env:USERPROFILE\.cargo\bin;$env:Path"
& rustup component add llvm-tools
$zip=Join-Path $root 'xwin.zip'
& curl.exe -fL --retry 2 'https://github.com/Jake-Shadle/xwin/releases/download/0.6.5/xwin-0.6.5-x86_64-pc-windows-msvc.tar.gz' -o $zip
if($LASTEXITCODE -ne 0){throw 'Unable to download portable SDK tool'}
New-Item -ItemType Directory -Force -Path (Join-Path $root 'xwin-bin') | Out-Null
& tar.exe -xzf $zip -C (Join-Path $root 'xwin-bin')
if($LASTEXITCODE -ne 0){throw 'Unable to extract SDK tool'}
$xwin=Get-ChildItem -Path (Join-Path $root 'xwin-bin') -Filter xwin.exe -Recurse | Select-Object -First 1 -ExpandProperty FullName
Write-Host "Portable SDK downloader: $xwin"
& $xwin --accept-license --arch x86_64 --variant desktop splat --output (Join-Path $root 'xwin')
if($LASTEXITCODE -ne 0){throw 'Unable to download Windows SDK'}
Write-Host 'Portable compiler SDK is ready.'
