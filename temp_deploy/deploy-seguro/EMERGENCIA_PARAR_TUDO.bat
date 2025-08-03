@echo off
echo ============================================
echo   FC DATA API - PROCEDIMENTO DE EMERGENCIA
echo ============================================
echo.
echo ATENCAO: Use apenas se o deploy falhou e
echo precisa voltar rapido ao estado anterior!
echo.
echo Deseja continuar? (S/N)
choice /C SN /N
if errorlevel 2 exit /b 0

echo.
echo [1] Parando servico imediatamente...
nssm stop FCDataAPI >nul 2>&1
net stop FCDataAPI >nul 2>&1
taskkill /F /IM fc-data-api.exe >nul 2>&1
echo [OK] Servico parado

echo.
echo [2] Removendo servico...
nssm remove FCDataAPI confirm >nul 2>&1
echo [OK] Servico removido

echo.
echo [3] Limpando processos travados...
taskkill /F /IM fc-data-api.exe >nul 2>&1
echo [OK] Processos limpos

echo.
echo [4] Verificando backups disponiveis...
echo.
echo Backups encontrados:
dir /B backup\deploy_* 2>nul
echo.
echo Para restaurar, execute o ROLLBACK.bat
echo do backup desejado.
echo.
pause
