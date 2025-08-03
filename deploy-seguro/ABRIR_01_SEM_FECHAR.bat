@echo off
REM Este arquivo abre o script principal sem fechar a janela

echo Abrindo Pre-Validacao em modo permanente...
echo.
cd /d "%~dp0"
cmd /k 01_PRE_VALIDACAO.bat
