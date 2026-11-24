$ErrorActionPreference='Stop'
$setup=Join-Path ${env:ProgramFiles(x86)} 'Microsoft Visual Studio\Installer\setup.exe'
$install=Join-Path ${env:ProgramFiles(x86)} 'Microsoft Visual Studio\2022\BuildTools'
& $setup modify --installPath $install --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended --passive --norestart --wait
Write-Host "Visual C++ workload result: $LASTEXITCODE"
