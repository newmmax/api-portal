@echo off
echo ========================================
echo TESTE DA CORRECAO - Query/SQL Fields
echo ========================================
echo.

set BASE_URL=http://localhost:8089/services/api1

echo 1. Testando health check...
curl -X GET "%BASE_URL%/health"
echo.
echo.

echo 2. Fazendo login...
curl -X POST "%BASE_URL%/auth/login" ^
  -H "Content-Type: application/json" ^
  -d "{\"username\":\"admin\",\"password\":\"ArtesanalFC2025!\"}" ^
  > token.tmp

echo.
echo Token salvo em token.tmp
echo.

echo IMPORTANTE: Para testar as queries personalizadas:
echo.
echo 1. PostgreSQL (aceita "query" e "sql"):
echo    POST /data/query
echo    Body: { "query": "SELECT * FROM fc14000 LIMIT 5" }
echo.
echo 2. Portal (aceita "query" e "sql"):
echo    POST /portal/query
echo    Body: { "query": "SELECT TOP 5 * FROM pedidos" }
echo.
echo 3. Protheus (aceita "query" e "sql"):
echo    POST /protheus/query
echo    Body: { "query": "SELECT TOP 5 * FROM SA1990" }
echo.
pause