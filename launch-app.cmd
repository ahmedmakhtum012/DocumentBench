@echo off
setlocal
set "APP_DIR=%~dp0"
set "APP_EXE=%APP_DIR%target\debug\documentbench.exe"
if not exist "%APP_EXE%" (
  echo DocumentBench has not been built yet. Building the desktop application...
  call "%APP_DIR%build-app.cmd" || exit /b 1
)
if not exist "%APP_EXE%" (
  echo The desktop executable was not created: %APP_EXE%
  exit /b 1
)
start "DocumentBench" "%APP_EXE%" %*
endlocal
