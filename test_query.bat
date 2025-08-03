@echo off
echo ===================================
echo FC Data API - Teste de Query
echo ===================================
echo.

REM Detectar porta
for /f "tokens=2 delims==" %%i in ('findstr "SERVER_PORT" .env') do set PORT=%%i
if "%PORT%"=="" set PORT=8080

set API_URL=http://localhost:%PORT%/services/api1

echo URL da API: %API_URL%
echo.

REM 1. Health Check
echo [1] Health Check...
echo -------------------
curl -X GET %API_URL%/health
echo.
echo.

REM 2. Login
echo [2] Login...
echo ------------
curl -X POST %API_URL%/auth/login ^
  -H "Content-Type: application/json" ^
  -d "{\"username\":\"admin\",\"password\":\"ArtesanalFC2025!\"}" ^
  -o token.json
echo.
type token.json
echo.
echo.

REM Extrair token
for /f "delims=" %%i in ('powershell -Command "(Get-Content token.json | ConvertFrom-Json).token"') do set TOKEN=%%i

if "%TOKEN%"=="" (
    echo ERRO: Token nao obtido!
    pause
    exit /b 1
)

REM 3. Testar endpoint de vendas SEM filtros
echo [3] Testando vendas SEM filtros (limite 5)...
echo ---------------------------------------------
curl -X GET "%API_URL%/data/vendas?limite=5" ^
  -H "Authorization: Bearer %TOKEN%"
echo.
echo.

REM 4. Testar endpoint de vendas COM filtros de data
echo [4] Testando vendas COM filtros de data...
echo ------------------------------------------
curl -X GET "%API_URL%/data/vendas?data_inicio=2024-01-01&data_fim=2025-12-31&limite=10" ^
  -H "Authorization: Bearer %TOKEN%"
echo.
echo.

REM 5. Testar endpoint de vendas detalhadas
echo [5] Testando vendas detalhadas...
echo ---------------------------------
curl -X GET "%API_URL%/data/vendas/detalhes?limite=5" ^
  -H "Authorization: Bearer %TOKEN%"
echo.
echo.

del token.json 2>nul

echo.
echo =========================================
echo Se houver erros, verifique:
echo 1. A API esta rodando? (test_api.bat)
echo 2. Os tipos de dados estao corretos?
echo 3. A query esta correta no PostgreSQL?
echo =========================================
pause
