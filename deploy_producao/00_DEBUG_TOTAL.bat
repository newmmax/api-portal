@echo off
REM ================================================
REM DEBUG - NAO FECHA DE JEITO NENHUM
REM ================================================
color 0E

echo DEBUG INICIANDO...
pause

cls
echo ========================================
echo    FC DATA API - DEBUG TOTAL
echo ========================================
echo.
echo Sistema: Windows Server 2012 R2
echo.
pause

echo Passo 1: Verificando diretorio atual...
echo.
echo Voce esta em:
cd
echo.
pause

echo Passo 2: Listando TODOS os arquivos...
echo.
dir
echo.
pause

echo Passo 3: Procurando por EXE...
echo.
dir *.exe
echo.
pause

echo Passo 4: Procurando por ENV...
echo.
dir *.env
echo.
pause

echo Passo 5: Verificando se e Admin...
echo.
net session >nul 2>&1
if %errorlevel% eq 0 (
    echo [OK] Voce E administrador
) else (
    echo [ERRO] Voce NAO e administrador
)
echo.
pause

echo Passo 6: Testando existencia de arquivos...
echo.
if exist "fc-data-api.exe" (
    echo [OK] fc-data-api.exe EXISTE
) else (
    echo [ERRO] fc-data-api.exe NAO EXISTE
)
echo.
if exist ".env" (
    echo [OK] .env EXISTE
) else (
    echo [ERRO] .env NAO EXISTE
)
echo.
pause

echo ========================================
echo FIM DO DEBUG
echo ========================================
echo.
echo Se o script fechou antes daqui, pode ser:
echo 1. Antivirus bloqueando
echo 2. Politica de execucao
echo 3. Problema de permissao
echo.
pause
pause
pause