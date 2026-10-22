@echo off
setlocal
cd /d "%~dp0"
if not exist "%~dp0Quantimate.exe" (
  echo Quantimate.exe was not found next to this launcher.
  pause
  exit /b 1
)
start "" "%~dp0Quantimate.exe"
timeout /t 3 /nobreak >nul
start "" "http://127.0.0.1:8765/"
echo Quantimate should be available at http://127.0.0.1:8765/
echo Close the Quantimate server window to stop the application.
