$ErrorActionPreference = 'Continue'
& winget.exe install --exact --id Microsoft.VisualStudio.2022.BuildTools --force --silent --accept-package-agreements --accept-source-agreements --override '--wait --passive --norestart --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended'
Write-Host "Build Tools installer result: $LASTEXITCODE"
