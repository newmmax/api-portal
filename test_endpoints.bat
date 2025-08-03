@echo off
echo ===================================
echo FC Data API - Teste de Endpoints
echo ===================================
echo.

REM Tentar detectar a porta do arquivo .env
for /f "tokens=2 delims==" %%i in ('findstr "SERVER_PORT" .env') do set PORT=%%i

REM Se não encontrou, usar porta padrão
if "%PORT%"=="" (
    set PORT=8080
    echo Usando porta padrao: 8080
) else (
    echo Usando porta configurada: %PORT%
)

set API_URL=http://localhost:%PORT%/services/api1
echo.
echo URL da API: %API_URL%
echo ===================================
echo.

REM Health Check
echo 1. Testando Health Check...
echo --------------------------
curl -X GET %API_URL%/health
echo.
echo.

REM Login
echo 2. Fazendo Login...
echo -------------------
curl -X POST %API_URL%/auth/login ^
  -H "Content-Type: application/json" ^
  -d "{\"username\":\"admin\",\"password\":\"ArtesanalFC2025!\"}" ^
  -o token.json
echo.
type token.json
echo.
echo.

REM Extrair token manualmente (PowerShell)
echo Extraindo token...
for /f "delims=" %%i in ('powershell -Command "(Get-Content token.json | ConvertFrom-Json).token"') do set TOKEN=%%i

if "%TOKEN%"=="" (
    echo ERRO: Nao foi possivel obter o token!
    echo Verifique se a API esta rodando na porta %PORT%
    pause
    exit /b 1
)

echo Token obtido com sucesso!
echo.

REM Validar token
echo 3. Validando Token...
echo ---------------------
curl -X GET %API_URL%/auth/validate ^
  -H "Authorization: Bearer %TOKEN%"
echo.
echo.

REM Consultar vendas
echo 4. Consultando Vendas (limite 5)...
echo -----------------------------------
curl -X GET "%API_URL%/data/vendas?limite=5" ^
  -H "Authorization: Bearer %TOKEN%"
echo.
echo.

REM Consultar vendas detalhadas
echo 5. Consultando Vendas Detalhadas (limite 10)...
echo -----------------------------------------------
curl -X GET "%API_URL%/data/vendas/detalhes?limite=10" ^
  -H "Authorization: Bearer %TOKEN%"
echo.
echo.

REM Query customizada
echo 6. Testando Query Customizada...
echo --------------------------------
curl -X POST %API_URL%/data/query ^
  -H "Authorization: Bearer %TOKEN%" ^
  -H "Content-Type: application/json" ^
  -d "{\"query\":\"SELECT COUNT(*) as total FROM fc14100\"}"
echo.
echo.

REM Limpar arquivo temporário
del token.json 2>nul

echo.
echo Testes concluidos!
echo Se algum teste falhou, verifique:
echo - A API esta rodando? (execute test_api.bat)
echo - A porta esta correta? (configurada: %PORT%)
echo - O banco PostgreSQL esta acessivel?
pause
