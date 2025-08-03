@echo off
echo =========================================================
echo   PASTA temp_deploy PRONTA PARA DEPLOY!
echo =========================================================
echo.
echo Local: C:\XAMPP\htdocs\portaldepedidos\fc-data-api\temp_deploy
echo.
echo CONTEUDO:
echo ---------
dir /B C:\XAMPP\htdocs\portaldepedidos\fc-data-api\temp_deploy
echo.
echo TAMANHO TOTAL:
echo --------------
dir C:\XAMPP\htdocs\portaldepedidos\fc-data-api\temp_deploy\*.* | findstr "File(s)"
echo.
echo PROXIMOS PASSOS:
echo ================
echo 1. IMPORTANTE: Edite temp_deploy\.env.producao
echo    - Mude JWT_SECRET
echo    - Mude ADMIN_PASSWORD
echo    - Renomeie para .env
echo.
echo 2. Copie a pasta temp_deploy para o servidor
echo    - Via rede, pendrive, etc
echo    - Destino sugerido: C:\temp_deploy
echo.
echo 3. No servidor, execute (como Admin):
echo    cd C:\temp_deploy\deploy-seguro
echo    01_VALIDACAO_MENU.bat
echo.
echo 4. Siga as instrucoes em LEIA_PRIMEIRO.txt
echo.
echo Abrir a pasta agora? (S/N)
choice /C SN /N
if errorlevel 1 (
    explorer C:\XAMPP\htdocs\portaldepedidos\fc-data-api\temp_deploy
)
