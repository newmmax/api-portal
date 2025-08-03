@echo off
echo ========================================
echo TESTE QUERY PORTAL - LOCAL
echo ========================================
echo.

set TOKEN=

echo 1. Fazendo login local...
for /f "tokens=*" %%i in ('curl -s -X POST "http://localhost:8089/services/api1/auth/login" -H "Content-Type: application/json" -d "{\"username\":\"admin\",\"password\":\"ArtesanalFC2025!\"}" 2^>nul ^| findstr "token"') do set RESPONSE=%%i

echo Response: %RESPONSE%
echo.

echo 2. Testando query no Portal (localhost)...
echo.
echo Query: SELECT TOP 5 * FROM clientes
echo.

curl -X POST "http://localhost:8089/services/api1/portal/query" ^
  -H "Authorization: Bearer SEU_TOKEN_AQUI" ^
  -H "Content-Type: application/json" ^
  -d "{\"sql\":\"SELECT TOP 5 * FROM clientes\"}" ^
  -m 30

echo.
echo.
echo 3. Testando query mais simples...
echo Query: SELECT COUNT(*) as total FROM clientes
echo.

curl -X POST "http://localhost:8089/services/api1/portal/query" ^
  -H "Authorization: Bearer SEU_TOKEN_AQUI" ^
  -H "Content-Type: application/json" ^
  -d "{\"sql\":\"SELECT COUNT(*) as total FROM clientes\"}" ^
  -m 30

echo.
echo.
echo IMPORTANTE: Substitua SEU_TOKEN_AQUI pelo token obtido no login!
echo.
pause