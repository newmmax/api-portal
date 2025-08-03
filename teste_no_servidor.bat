REM Execute este comando NO SERVIDOR onde a API está rodando

@echo off
echo Testando query local no servidor...
echo.

curl -X POST "http://localhost:8089/services/api1/portal/query" ^
  -H "Authorization: Bearer %1" ^
  -H "Content-Type: application/json" ^
  -d "{\"sql\":\"SELECT 1 as teste\"}" ^
  -v

echo.
echo Se der erro aqui, o problema é:
echo 1. API não está rodando (verificar com: netstat -an | findstr 8089)
echo 2. SQL Server não está acessível do servidor
echo 3. Configuração errada no .env
echo.
echo Ver logs em: C:\fcdata-api\logs\service.log
pause