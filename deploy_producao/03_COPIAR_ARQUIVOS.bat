@echo off
REM ================================================
REM PASSO 3: COPIAR ARQUIVOS PARA PRODUCAO
REM ================================================
color 0E
cls

echo ========================================
echo    FC DATA API - COPIAR ARQUIVOS
echo ========================================
echo.

REM Verificar se executavel existe
if not exist "fc-data-api.exe" (
    color 0C
    echo [ERRO] fc-data-api.exe nao encontrado nesta pasta!
    echo.
    echo Diretorio atual: %CD%
    echo.
    pause
    exit /b 1
)

if not exist ".env" (
    color 0C
    echo [ERRO] .env nao encontrado nesta pasta!
    pause
    exit /b 1
)

REM Fazer backup se ja existir
if exist "C:\fcdata-api\app\fc-data-api.exe" (
    echo Fazendo backup da versao anterior...
    set backup_name=fc-data-api_%date:~0,2%-%date:~3,2%-%date:~6,4%_%time:~0,2%-%time:~3,2%.exe
    set backup_name=%backup_name: =0%
    copy "C:\fcdata-api\app\fc-data-api.exe" "C:\fcdata-api\backup\%backup_name%" >nul
    echo [OK] Backup criado
)

REM Copiar arquivos
echo.
echo Copiando arquivos...
copy /Y "fc-data-api.exe" "C:\fcdata-api\app\" >nul
if %errorlevel% eq 0 (
    echo [OK] fc-data-api.exe copiado
) else (
    color 0C
    echo [ERRO] Falha ao copiar fc-data-api.exe
    pause
    exit /b 1
)

copy /Y ".env" "C:\fcdata-api\app\" >nul
if %errorlevel% eq 0 (
    echo [OK] .env copiado
) else (
    color 0C
    echo [ERRO] Falha ao copiar .env
    pause
    exit /b 1
)

echo.
echo Arquivos instalados em: C:\fcdata-api\app\
echo.

color 0A
echo ========================================
echo    ARQUIVOS COPIADOS COM SUCESSO!
echo ========================================
echo.
echo Proximo passo: Execute 04_TESTE_MANUAL.bat
echo.
pause