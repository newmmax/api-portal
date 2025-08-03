@echo off
REM ================================================
REM PASSO 7: INICIAR SERVIÇO
REM ================================================
color 0E
cls

echo ========================================
echo    FC DATA API - INICIAR SERVIÇO
echo ========================================
echo.

echo Iniciando serviço FCDataAPI...
echo.

REM Iniciar serviço
C:\fcdata-api\tools\nssm.exe start FCDataAPI

REM Aguardar um pouco
timeout /t 3 >nul

REM Verificar status
echo.
echo Verificando status do serviço...
sc query FCDataAPI | findstr /i "running"

if %errorlevel% eq 0 (
    color 0A
    echo.
    echo [OK] Serviço está rodando!
) else (
    color 0C
    echo.
    echo [ERRO] Serviço não está rodando!
    echo.
    echo Verificando logs...
    type C:\fcdata-api\logs\error.log 2>nul
    echo.
    echo Verifique os logs em: C:\fcdata-api\logs\
    pause
    exit /b 1
)

echo.
echo Testando API...
echo.

REM Teste simples
curl -s http://127.0.0.1:8089/services/api1/health >nul 2>&1
if %errorlevel% eq 0 (
    echo [OK] API está respondendo!
    echo.
    curl http://127.0.0.1:8089/services/api1/health
) else (
    echo [AVISO] API não respondeu ao teste
    echo Aguarde mais alguns segundos e tente novamente
)

echo.
color 0A
echo ========================================
echo    SERVIÇO INICIADO COM SUCESSO!
echo ========================================
echo.
echo Próximo passo: Execute 08_CONFIGURAR_APACHE.bat
echo.
pause