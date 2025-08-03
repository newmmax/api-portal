@echo off
echo ================================================
echo INVESTIGACAO PROFISSIONAL - PORTAL QUERY DEBUG
echo ================================================
echo.

set TOKEN=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhZG1pbl9wcm9kIiwiZXhwIjoxNzU0MTM3ODIyLCJpYXQiOjE3NTQwNTE0MjJ9.zbOytl3OPatV-0eQ1cvjTkVS1dIIoaEMqzccqUoWmSg

echo TESTE 1: Query baseline simples (JA CONFIRMADA FUNCIONANDO)
echo curl -X POST http://localhost:8089/services/api1/portal/query -H "Authorization: Bearer %TOKEN%" -d "{\"sql\": \"SELECT 1 as test\"}"
curl -X POST http://localhost:8089/services/api1/portal/query ^
  -H "Content-Type: application/json" ^
  -H "Authorization: Bearer %TOKEN%" ^
  -d "{\"sql\": \"SELECT 1 as test\"}"
echo.
echo.

echo TESTE 2: Verificar se tabela clientes existe
echo curl -X POST http://localhost:8089/services/api1/portal/query -H "Authorization: Bearer %TOKEN%" -d "{\"sql\": \"SELECT COUNT(*) as total FROM clientes\"}"
curl -X POST http://localhost:8089/services/api1/portal/query ^
  -H "Content-Type: application/json" ^
  -H "Authorization: Bearer %TOKEN%" ^
  -d "{\"sql\": \"SELECT COUNT(*) as total FROM clientes\"}"
echo.
echo.

echo TESTE 3: Query simples com campos basicos
echo curl -X POST http://localhost:8089/services/api1/portal/query -H "Authorization: Bearer %TOKEN%" -d "{\"sql\": \"SELECT TOP 1 id, nome FROM clientes\"}"
curl -X POST http://localhost:8089/services/api1/portal/query ^
  -H "Content-Type: application/json" ^
  -H "Authorization: Bearer %TOKEN%" ^
  -d "{\"sql\": \"SELECT TOP 1 id, nome FROM clientes\"}"
echo.
echo.

echo TESTE 4: Adicionar campos potencialmente problematicos
echo curl -X POST http://localhost:8089/services/api1/portal/query -H "Authorization: Bearer %TOKEN%" -d "{\"sql\": \"SELECT TOP 1 id, nome, email, cnpj FROM clientes\"}"
curl -X POST http://localhost:8089/services/api1/portal/query ^
  -H "Content-Type: application/json" ^
  -H "Authorization: Bearer %TOKEN%" ^
  -d "{\"sql\": \"SELECT TOP 1 id, nome, email, cnpj FROM clientes\"}"
echo.
echo.

echo TESTE 5: Testar campos de data com CONVERT
echo curl -X POST http://localhost:8089/services/api1/portal/query -H "Authorization: Bearer %TOKEN%" -d "{\"sql\": \"SELECT TOP 1 id, CONVERT(varchar, created_at, 120) as created_at FROM clientes\"}"
curl -X POST http://localhost:8089/services/api1/portal/query ^
  -H "Content-Type: application/json" ^
  -H "Authorization: Bearer %TOKEN%" ^
  -d "{\"sql\": \"SELECT TOP 1 id, CONVERT(varchar, created_at, 120) as created_at FROM clientes\"}"
echo.
echo.

echo TESTE 6: Testar campo booleano com CAST
echo curl -X POST http://localhost:8089/services/api1/portal/query -H "Authorization: Bearer %TOKEN%" -d "{\"sql\": \"SELECT TOP 1 id, CAST(is_first_login as int) as is_first_login FROM clientes\"}"
curl -X POST http://localhost:8089/services/api1/portal/query ^
  -H "Content-Type: application/json" ^
  -H "Authorization: Bearer %TOKEN%" ^
  -d "{\"sql\": \"SELECT TOP 1 id, CAST(is_first_login as int) as is_first_login FROM clientes\"}"
echo.
echo.

echo TESTE 7: Query original COMPLETA (a que falha)
echo curl -X POST http://localhost:8089/services/api1/portal/query -H "Authorization: Bearer %TOKEN%" -d "{\"sql\": \"SELECT id, cod_totvs, loja, nome, email, cnpj, cidade, estado, CONVERT(varchar, created_at, 120) as created_at, CONVERT(varchar, updated_at, 120) as updated_at, CONVERT(varchar, deleted_at, 120) as deleted_at, CAST(is_first_login as int) as is_first_login, grupo_venda, nome_fantasia FROM clientes\"}"
curl -X POST http://localhost:8089/services/api1/portal/query ^
  -H "Content-Type: application/json" ^
  -H "Authorization: Bearer %TOKEN%" ^
  -d "{\"sql\": \"SELECT id, cod_totvs, loja, nome, email, cnpj, cidade, estado, CONVERT(varchar, created_at, 120) as created_at, CONVERT(varchar, updated_at, 120) as updated_at, CONVERT(varchar, deleted_at, 120) as deleted_at, CAST(is_first_login as int) as is_first_login, grupo_venda, nome_fantasia FROM clientes\"}"
echo.
echo.

echo ================================================
echo ANALISE COMPLETA
echo ================================================
echo.
echo INSTRUCOES:
echo 1. Observe qual teste falha primeiro
echo 2. O ultimo teste que funciona indica onde esta o problema
echo 3. Compare campos entre teste que funciona vs teste que falha
echo 4. Identifique tipo SQL Server problematico
echo.
pause
