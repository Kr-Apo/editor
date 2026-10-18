@echo off
cd /d "%~dp0"
where node >nul 2>nul
if errorlevel 1 (
  echo Cette demonstration demande Node.js 22 ou plus recent.
  echo Consultez README.md. Le projet Ankhimate et les exports restent utilisables sans Node.js.
  pause
  exit /b 1
)
set ANKHIMATE_PORT=4177
echo Ouvrez http://127.0.0.1:4177 dans votre navigateur.
echo Gardez cette fenetre ouverte pendant la demonstration.
node server.mjs
pause
