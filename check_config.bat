@echo off
echo ===================================
echo FC Data API - Verificacao de Config
echo ===================================
echo.

echo Configuracao atual (.env):
echo --------------------------
findstr "SERVER_PORT" .env
findstr "SERVER_HOST" .env
findstr "API_PREFIX" .env
echo.

REM Detectar porta
for /f "tokens=2 delims==" %%i in ('findstr "SERVER_PORT" .env') do set PORT=%%i
if "%PORT%"=="" set PORT=8080

echo URLs de acesso:
echo ---------------
echo Local: http://localhost:%PORT%/services/api1
echo Producao: https://conexao.artesanalfarmacia.com.br/services/api1
echo.

echo Verificando se a porta esta em uso...
netstat -ano | findstr :%PORT% >nul
if %errorlevel%==0 (
    echo.
    echo ATENCAO: Porta %PORT% ja esta em uso!
    echo Processos usando a porta:
    netstat -ano | findstr :%PORT%
    echo.
    echo Se a API nao estiver rodando, mate o processo ou mude a porta.
) else (
    echo Porta %PORT% esta livre!
)

echo.
echo Proximos passos:
echo 1. Execute test_api.bat para iniciar a API
echo 2. Execute test_endpoints.bat para testar os endpoints
echo 3. Importe a colecao Postman para testes completos
echo.
pause
