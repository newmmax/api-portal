@echo off
echo ============================================
echo   FC DATA API - BACKUP DE CONFIGURACOES
echo ============================================
echo.

REM Criar estrutura de backup com timestamp
for /f "tokens=2 delims==" %%a in ('wmic OS Get localdatetime /value') do set "dt=%%a"
set "timestamp=%dt:~0,4%-%dt:~4,2%-%dt:~6,2%_%dt:~8,2%-%dt:~10,2%"
set BACKUP_DIR=backup\deploy_%timestamp%

echo Criando diretorio de backup: %BACKUP_DIR%
mkdir "%BACKUP_DIR%" 2>nul
mkdir "%BACKUP_DIR%\apache" 2>nul
mkdir "%BACKUP_DIR%\servico" 2>nul

echo.
echo [1/5] Fazendo backup do .env atual...
if exist ".env" (
    copy ".env" "%BACKUP_DIR%\.env.bak" >nul
    echo [OK] .env salvo
) else (
    echo [AVISO] .env nao encontrado
)

echo.
echo [2/5] Fazendo backup de configuracoes Apache...
if exist "C:\XAMPP\apache\conf\extra\httpd-vhosts.conf" (
    copy "C:\XAMPP\apache\conf\extra\httpd-vhosts.conf" "%BACKUP_DIR%\apache\httpd-vhosts.conf.bak" >nul
    echo [OK] httpd-vhosts.conf salvo
) else (
    echo [AVISO] httpd-vhosts.conf nao encontrado
)

echo.
echo [3/5] Verificando servico existente...
sc query FCDataAPI >nul 2>&1
if %errorlevel% == 0 (
    echo [INFO] Servico FCDataAPI existe, salvando configuracao...
    nssm dump FCDataAPI > "%BACKUP_DIR%\servico\FCDataAPI_config.txt" 2>nul
    sc query FCDataAPI > "%BACKUP_DIR%\servico\FCDataAPI_status.txt" 2>nul
    echo [OK] Configuracao do servico salva
) else (
    echo [INFO] Nenhum servico para fazer backup
)

echo.
echo [4/5] Fazendo backup do executavel atual (se existir em C:\fcdata-api)...
if exist "C:\fcdata-api\fc-data-api.exe" (
    copy "C:\fcdata-api\fc-data-api.exe" "%BACKUP_DIR%\fc-data-api.exe.bak" >nul
    echo [OK] Executavel salvo
    if exist "C:\fcdata-api\.env" (
        copy "C:\fcdata-api\.env" "%BACKUP_DIR%\.env.producao.bak" >nul
        echo [OK] .env de producao salvo
    )
) else (
    echo [INFO] Nenhuma instalacao anterior encontrada
)

echo.
echo [5/5] Criando script de rollback...
(
echo @echo off
echo echo ============================================
echo echo   FC DATA API - ROLLBACK AUTOMATICO
echo echo ============================================
echo echo.
echo echo Este script restaura as configuracoes anteriores
echo echo.
echo.
echo echo [1/4] Parando servico...
echo nssm stop FCDataAPI ^>nul 2^>^&1
echo net stop FCDataAPI ^>nul 2^>^&1
echo timeout /t 2 /nobreak ^>nul
echo.
echo echo [2/4] Removendo servico...
echo nssm remove FCDataAPI confirm ^>nul 2^>^&1
echo.
echo echo [3/4] Restaurando arquivos...
echo if exist "%BACKUP_DIR%\fc-data-api.exe.bak" ^(
echo     copy "%BACKUP_DIR%\fc-data-api.exe.bak" "C:\fcdata-api\fc-data-api.exe" /Y ^>nul
echo     echo [OK] Executavel restaurado
echo ^)
echo if exist "%BACKUP_DIR%\.env.producao.bak" ^(
echo     copy "%BACKUP_DIR%\.env.producao.bak" "C:\fcdata-api\.env" /Y ^>nul
echo     echo [OK] .env restaurado
echo ^)
echo if exist "%BACKUP_DIR%\apache\httpd-vhosts.conf.bak" ^(
echo     copy "%BACKUP_DIR%\apache\httpd-vhosts.conf.bak" "C:\XAMPP\apache\conf\extra\httpd-vhosts.conf" /Y ^>nul
echo     echo [OK] Apache config restaurado
echo ^)
echo.
echo echo [4/4] Reiniciando Apache...
echo C:\XAMPP\apache\bin\httpd.exe -k restart
echo.
echo echo.
echo echo Rollback concluido!
echo pause
) > "%BACKUP_DIR%\ROLLBACK.bat"

echo.
echo ============================================
echo   BACKUP CONCLUIDO!
echo ============================================
echo.
echo Backup salvo em: %BACKUP_DIR%
echo.
echo Conteudo do backup:
dir /B "%BACKUP_DIR%"
echo.
echo IMPORTANTE: Em caso de problemas, execute:
echo %BACKUP_DIR%\ROLLBACK.bat
echo.
echo Proximo passo: Execute 03_DEPLOY_PASSO_A_PASSO.bat
echo.
pause
