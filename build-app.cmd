@echo off
call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\Common7\Tools\VsDevCmd.bat" -no_logo -arch=x64 -host_arch=x64 || exit /b 1
set "PATH=%~dp0.tools\node-v22.18.0-win-x64;%USERPROFILE%\.cargo\bin;%PATH%"
call npm.cmd run build || exit /b 1
cargo build --manifest-path "%~dp0src-tauri\Cargo.toml" || exit /b 1
cargo test --manifest-path "%~dp0src-tauri\Cargo.toml" || exit /b 1
