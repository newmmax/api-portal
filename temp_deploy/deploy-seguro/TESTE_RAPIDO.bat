@echo off
echo ============================================
echo   FC DATA API - TESTE RAPIDO POS-DEPLOY
echo ============================================
echo.

set API_URL=http://localhost:8089/services/api1
set PROD_URL=https://conexao.artesanalfarmacia.com.br/services/api1

echo [1] Health Check Local...
curl -w "\nTempo: %%{time_total}s\n" %API_URL%/health
echo.

echo [2] Health Check HTTPS...
curl -k -w "\nTempo: %%{time_total}s\n" %PROD_URL%/health 2>nul
echo.

echo [3] Login Test...
curl -s -X POST %API_URL%/auth/login ^
  -H "Content-Type: application/json" ^
  -d "{\"username\":\"admin\",\"password\":\"ArtesanalFC2025!\"}" | findstr "token" >nul
if %errorlevel% == 0 (
    echo [OK] Login funcionando
) else (
    echo [ERRO] Login falhou!
)
echo.

echo [4] Performance basica...
echo Fazendo 5 requisicoes...
for /L %%i in (1,1,5) do (
    curl -s -w "Req %%i: %%{time_total}s\n" -o nul %API_URL%/health
)
echo.

echo ============================================
echo Se todos os testes passaram, o deploy foi
echo bem sucedido!
echo ============================================
pause
