@echo off
echo ========================================
echo DIAGNOSTICO FC DATA API - PRODUCAO
echo ========================================
echo.

echo 1. Testando API localmente (porta 8089)...
echo.
curl -X GET "http://localhost:8089/services/api1/health" -m 5
echo.
echo.

echo 2. Verificando se a porta 8089 esta escutando...
netstat -an | findstr :8089
echo.

echo 3. Testando conexao com SQL Server Portal...
echo.
echo Criando teste_sql_portal.js...
(
echo const sql = require^('mssql'^);
echo.
echo const config = {
echo     server: '10.216.1.11',
echo     port: 1433,
echo     database: 'sys_pedidos',
echo     user: 'sa',
echo     password: '5y54dm1n%%',
echo     options: {
echo         encrypt: false,
echo         trustServerCertificate: true,
echo         connectTimeout: 5000,
echo         requestTimeout: 5000
echo     }
echo };
echo.
echo async function testar^(^) {
echo     try {
echo         console.log^('Conectando ao SQL Server Portal...'^);
echo         await sql.connect^(config^);
echo         console.log^('✓ Conectado com sucesso!'^);
echo         
echo         const result = await sql.query^('SELECT TOP 1 * FROM clientes'^);
echo         console.log^('✓ Query executada! Registros:', result.recordset.length^);
echo         
echo         await sql.close^(^);
echo     } catch ^(err^) {
echo         console.error^('✗ Erro:', err.message^);
echo     }
echo }
echo.
echo testar^(^);
) > teste_sql_portal.js

echo.
echo 4. Executando teste de conexao SQL...
node teste_sql_portal.js 2>nul || echo ERRO: Node.js nao encontrado ou erro no script
echo.

echo 5. Testando login na API local...
echo.
curl -X POST "http://localhost:8089/services/api1/auth/login" ^
  -H "Content-Type: application/json" ^
  -d "{\"username\":\"admin\",\"password\":\"ArtesanalFC2025!\"}" ^
  -m 10 > login_result.json 2>nul

if exist login_result.json (
    echo Login response saved to login_result.json
    type login_result.json
) else (
    echo ERRO: Nao foi possivel fazer login
)
echo.

echo 6. Verificando processos fc-data-api...
echo.
tasklist | findstr fc-data-api
echo.

echo 7. Ultimas linhas do log (se existir)...
echo.
if exist "C:\fcdata-api\logs\service.log" (
    echo Ultimas 20 linhas do log:
    powershell -Command "Get-Content 'C:\fcdata-api\logs\service.log' -Tail 20"
) else (
    echo Log nao encontrado em C:\fcdata-api\logs\service.log
)
echo.

echo ========================================
echo RESUMO DO DIAGNOSTICO
echo ========================================
echo.
echo Se a API nao responde localmente:
echo   - Servico pode estar parado
echo   - Executar: nssm start FCDataAPI
echo.
echo Se SQL Server nao conecta:
echo   - Verificar rede/firewall
echo   - Verificar credenciais
echo.
echo Se tudo funciona local mas nao em producao:
echo   - Problema no Apache proxy
echo   - Verificar configuracao do VirtualHost
echo.
pause

del teste_sql_portal.js 2>nul
del login_result.json 2>nul