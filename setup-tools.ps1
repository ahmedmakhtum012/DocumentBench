$ErrorActionPreference = 'Stop'
$root = Join-Path $PSScriptRoot '.tools'
New-Item -ItemType Directory -Force -Path $root | Out-Null
Write-Host 'Build tool locations:'
Get-Command curl.exe,link.exe,cl.exe -ErrorAction SilentlyContinue | Format-Table Name,Source -AutoSize
$cargo = Join-Path $env:USERPROFILE '.cargo\bin\cargo.exe'
if (-not (Test-Path $cargo)) {
  Write-Host 'Downloading the official Rust installer...'
  $installer = Join-Path $root 'rustup-init.exe'
  & curl.exe -fL --retry 2 'https://win.rustup.rs/x86_64' -o $installer
  if ($LASTEXITCODE -ne 0) { throw 'Rust installer download failed' }
  Write-Host 'Installing Rust (stable, minimal profile)...'
  & $installer -y --default-toolchain stable --profile minimal --no-modify-path
  if ($LASTEXITCODE -ne 0) { throw 'Rust installation failed' }
}
$nodeHome = Join-Path $root 'node-v22.18.0-win-x64'
if (-not (Test-Path (Join-Path $nodeHome 'node.exe'))) {
  Write-Host 'Downloading Node.js 22...'
  $archive = Join-Path $root 'node.zip'
  & curl.exe -fL --retry 2 'https://nodejs.org/dist/v22.18.0/node-v22.18.0-win-x64.zip' -o $archive
  if ($LASTEXITCODE -ne 0) { throw 'Node.js download failed' }
  Expand-Archive -LiteralPath $archive -DestinationPath $root -Force
}
$env:Path = (Join-Path $env:USERPROFILE '.cargo\bin') + ';' + $nodeHome + ';' + $env:Path
& $cargo --version
& (Join-Path $nodeHome 'node.exe') --version
Write-Host 'Rust and Node.js are ready.'
