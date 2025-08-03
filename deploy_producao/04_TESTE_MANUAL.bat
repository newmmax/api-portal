@echo off
REM ================================================
REM PASSO 4: TESTE MANUAL DA API
REM ================================================
color 0E
cls

echo ========================================
echo    FC DATA API - TESTE MANUAL
echo ========================================
echo.
echo Este teste vai executar a API diretamente
echo para verificar se está funcionando.
echo.
echo Pressione Ctrl+C para parar quando terminar o teste.
echo.
pause

cd /d C:\fcdata-api\app

echo.
echo Iniciando API...
echo.

fc-data-api.exe

echo.
echo API foi encerrada.
echo.
echo Se a API funcionou corretamente, execute 05_INSTALAR_SERVICO.bat
echo Se houve erros, verifique:
echo   - Configurações no arquivo .env
echo   - Conectividade com banco de dados
echo   - Portas em uso
echo.
pause