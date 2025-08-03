@echo off
REM ================================================
REM PASSO 5: INSTALAR COMO SERVICO WINDOWS
REM ================================================
color 0E
cls

echo ========================================
echo    FC DATA API - INSTALAR SERVICO
echo ========================================
echo.

REM Verificar se NSSM existe
if not exist "C:\fcdata-api\tools\nssm.exe" (
    echo [AVISO] NSSM nao encontrado!
    echo.
    echo Por favor, baixe o NSSM de: https://nssm.cc/download
    echo E extraia o nssm.exe (versao 64-bit) para: C:\fcdata-api\tools\
    echo.
    echo Apos baixar e extrair, execute este script novamente.
    echo.
    pause
    exit /b 1
)

REM Verificar se servico ja existe
sc query FCDataAPI >nul 2>&1
if %errorlevel% eq 0 (
    echo [INFO] Servico FCDataAPI ja existe. Removendo...
    C:\fcdata-api\tools\nssm.exe stop FCDataAPI >nul 2>&1
    timeout /t 2 >nul
    C:\fcdata-api\tools\nssm.exe remove FCDataAPI confirm >nul 2>&1
    timeout /t 3 >nul
)

echo Instalando servico...
echo.

REM Instalar servico
C:\fcdata-api\tools\nssm.exe install FCDataAPI "C:\fcdata-api\app\fc-data-api.exe"

if %errorlevel% neq 0 (
    echo [ERRO] Falha ao instalar servico
    pause
    exit /b 1
)

REM Configurar servico
echo Configurando servico...
C:\fcdata-api\tools\nssm.exe set FCDataAPI AppDirectory "C:\fcdata-api\app"
C:\fcdata-api\tools\nssm.exe set FCDataAPI DisplayName "FC Data API"
C:\fcdata-api\tools\nssm.exe set FCDataAPI Description "API unificada Formula Certa - PostgreSQL e SQL Server"
C:\fcdata-api\tools\nssm.exe set FCDataAPI Start SERVICE_AUTO_START

REM Configurar logs
C:\fcdata-api\tools\nssm.exe set FCDataAPI AppStdout "C:\fcdata-api\logs\service.log"
C:\fcdata-api\tools\nssm.exe set FCDataAPI AppStderr "C:\fcdata-api\logs\error.log"
C:\fcdata-api\tools\nssm.exe set FCDataAPI AppRotateFiles 1
C:\fcdata-api\tools\nssm.exe set FCDataAPI AppRotateOnline 1
C:\fcdata-api\tools\nssm.exe set FCDataAPI AppRotateBytes 10485760

echo.
echo [OK] Servico instalado!
echo.

color 0A
echo ========================================
echo    SERVICO INSTALADO COM SUCESSO!
echo ========================================
echo.
echo Proximo passo: Execute 06_CONFIGURAR_FIREWALL.bat
echo.
pause