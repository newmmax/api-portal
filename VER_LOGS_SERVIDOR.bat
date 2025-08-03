@echo off
echo Verificando logs da API no servidor...
echo.
echo Execute isso NO SERVIDOR onde a API esta rodando:
echo.
echo 1. Ver ultimas 50 linhas do log:
echo    type C:\fcdata-api\logs\service.log | more
echo.
echo 2. Ver erros especificos:
echo    findstr /i "error panic fatal" C:\fcdata-api\logs\service.log
echo.
echo 3. Monitorar log em tempo real (PowerShell):
echo    Get-Content C:\fcdata-api\logs\service.log -Wait -Tail 20
echo.
echo O erro 502 significa que a API travou ou demorou demais.
echo O log deve mostrar o erro exato!
pause