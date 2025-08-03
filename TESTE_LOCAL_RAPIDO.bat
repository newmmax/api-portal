@echo off
echo ========================================
echo TESTE RAPIDO - API LOCAL vs QUERY
echo ========================================
echo.

echo 1. A API esta rodando localmente?
curl -X GET "http://localhost:8089/services/api1/health" -m 5
echo.
echo.

echo 2. Fazendo login local...
curl -X POST "http://localhost:8089/services/api1/auth/login" ^
  -H "Content-Type: application/json" ^
  -d "{\"username\":\"admin\",\"password\":\"ArtesanalFC2025!\"}" ^
  -o token.txt -s

echo.
echo Token salvo em token.txt
echo.

echo 3. IMPORTANTE: Execute este comando com o token obtido acima:
echo.
echo curl -X POST "http://localhost:8089/services/api1/portal/query" ^
echo   -H "Authorization: Bearer TOKEN_AQUI" ^
echo   -H "Content-Type: application/json" ^
echo   -d "{\"sql\":\"SELECT TOP 1 * FROM clientes\"}"
echo.
echo Se funcionar local mas nao em producao, o problema pode ser:
echo - A API nao esta rodando
echo - O SQL Server nao esta acessivel da API
echo - A query esta causando erro interno na API
echo.
pause