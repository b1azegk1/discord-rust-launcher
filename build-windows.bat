@echo off
setlocal
cd /d "%~dp0"

echo ==========================================
echo   Discord Launcher - Windows x64 build
echo ==========================================
echo.

where node >nul 2>nul
if errorlevel 1 (
  echo [ERROR] Node.js 20+ is not installed.
  echo Install the LTS version from https://nodejs.org/
  goto :fail
)

where npm >nul 2>nul
if errorlevel 1 (
  echo [ERROR] npm was not found. Reinstall Node.js.
  goto :fail
)

where cargo >nul 2>nul
if errorlevel 1 (
  echo [ERROR] Rust is not installed.
  echo Install it from https://rustup.rs/ and restart this window.
  goto :fail
)

echo [1/3] Installing frontend dependencies...
call npm install
if errorlevel 1 goto :fail

echo.
echo [2/3] Building Discord Launcher...
call npm run tauri build
if errorlevel 1 goto :fail

echo.
echo [3/3] Build completed.
echo Output folder:
echo %CD%\src-tauri\target\release\bundle
echo.
echo Installer files:
dir /b "%CD%\src-tauri\target\release\bundle\nsis\*.exe" 2>nul
dir /b "%CD%\src-tauri\target\release\bundle\msi\*.msi" 2>nul
echo.
pause
exit /b 0

:fail
echo.
echo [ERROR] Build failed. Check the messages above.
echo Requirements: Node.js 20+, Rust stable, Microsoft C++ Build Tools,
echo and WebView2 Runtime.
pause
exit /b 1
