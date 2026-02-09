@echo off
setlocal

set "SCRIPT_DIR=%~dp0"
for %%I in ("%SCRIPT_DIR%..") do set "REPO_ROOT=%%~fI"

clang -I"%REPO_ROOT%\wasm-sysroot" -I"%REPO_ROOT%\wasm-sysroot\src" -I"%REPO_ROOT%\wasm-sysroot\sys" %*
exit /b %ERRORLEVEL%
