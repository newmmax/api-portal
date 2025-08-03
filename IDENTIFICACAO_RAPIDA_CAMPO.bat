@echo off
echo ================================================
echo IDENTIFICACAO RAPIDA - CAMPO PROBLEMATICO
echo ================================================
echo.

set TOKEN=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhZG1pbl9wcm9kIiwiZXhwIjoxNzU0MTM3ODIyLCJpYXQiOjE3NTQwNTE0MjJ9.zbOytl3OPatV-0eQ1cvjTkVS1dIIoaEMqzccqUoWmSg
set URL=https://conexao.artesanalfarmacia.com.br/services/api1/portal/query

echo TESTE 1: Campo ID (Primary Key - INT/BIGINT)
curl -s -X POST "%URL%" ^
  -H "Content-Type: application/json" ^
  -H "Authorization: Bearer %TOKEN%" ^
  -d "{\"query\": \"SELECT TOP 1 nome, id FROM clientes\"}" | findstr /i "success\|error"
echo.

echo TESTE 2: Campo created_at (DATETIME)
curl -s -X POST "%URL%" ^
  -H "Content-Type: application/json" ^
  -H "Authorization: Bearer %TOKEN%" ^
  -d "{\"query\": \"SELECT TOP 1 nome, created_at FROM clientes\"}" | findstr /i "success\|error"
echo.

echo TESTE 3: Campo is_first_login (BIT/BOOLEAN)
curl -s -X POST "%URL%" ^
  -H "Content-Type: application/json" ^
  -H "Authorization: Bearer %TOKEN%" ^
  -d "{\"query\": \"SELECT TOP 1 nome, is_first_login FROM clientes\"}" | findstr /i "success\|error"
echo.

echo TESTE 4: Campo deleted_at (DATETIME NULL)
curl -s -X POST "%URL%" ^
  -H "Content-Type: application/json" ^
  -H "Authorization: Bearer %TOKEN%" ^
  -d "{\"query\": \"SELECT TOP 1 nome, deleted_at FROM clientes\"}" | findstr /i "success\|error"
echo.

echo TESTE 5: Campo cnpj (VARCHAR formatado)
curl -s -X POST "%URL%" ^
  -H "Content-Type: application/json" ^
  -H "Authorization: Bearer %TOKEN%" ^
  -d "{\"query\": \"SELECT TOP 1 nome, cnpj FROM clientes\"}" | findstr /i "success\|error"
echo.

echo TESTE 6: Combinacao de 3 campos basicos
curl -s -X POST "%URL%" ^
  -H "Content-Type: application/json" ^
  -H "Authorization: Bearer %TOKEN%" ^
  -d "{\"query\": \"SELECT TOP 1 id, nome, email FROM clientes\"}" | findstr /i "success\|error"
echo.

echo TESTE 7: SELECT * (deve falhar)
curl -s -X POST "%URL%" ^
  -H "Content-Type: application/json" ^
  -H "Authorization: Bearer %TOKEN%" ^
  -d "{\"query\": \"SELECT * FROM clientes\"}" | findstr /i "success\|error"
echo.

echo ================================================
echo ANALISE DOS RESULTADOS:
echo ================================================
echo.
echo Se aparecer "success":true = Campo OK
echo Se aparecer erro HTTP ou "success":false = Campo PROBLEMATICO
echo.
echo O primeiro teste que falhar indica o campo que causa problema
echo.
pause
