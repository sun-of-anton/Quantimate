@echo off
setlocal
cd /d "%~dp0"
if not exist "%~dp0Quantimate.exe" (
  echo Quantimate.exe was not found. Extract the entire download ZIP before launching.
  pause
  exit /b 1
)
if defined LOCALAPPDATA (
  set "QUANTIMATE_DATA=%LOCALAPPDATA%\Quantimate"
) else (
  set "QUANTIMATE_DATA=%~dp0data"
)
if not exist "%QUANTIMATE_DATA%" mkdir "%QUANTIMATE_DATA%"
if errorlevel 1 (
  echo Could not create the local Quantimate data directory.
  pause
  exit /b 1
)
start "Quantimate - local server" /D "%~dp0" "%~dp0Quantimate.exe"
timeout /t 2 /nobreak >nul
start "" "http://127.0.0.1:8765/"
exit /b 0
