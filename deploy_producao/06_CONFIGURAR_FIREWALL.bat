@echo off
REM ================================================
REM PASSO 6: CONFIGURAR FIREWALL
REM ================================================
color 0E
cls

echo ========================================
echo    FC DATA API - CONFIGURAR FIREWALL
echo ========================================
echo.
echo IMPORTANTE: Vamos configurar o firewall para
echo permitir APENAS acesso LOCAL (127.0.0.1)
echo.

REM Remover regra antiga se existir
echo Removendo regras antigas...
netsh advfirewall firewall delete rule name="FC Data API Local" >nul 2>&1
netsh advfirewall firewall delete rule name="FC Data API" >nul 2>&1

echo.
echo Adicionando nova regra de firewall...

REM Adicionar regra APENAS para localhost
netsh advfirewall firewall add rule ^
    name="FC Data API Local" ^
    dir=in ^
    action=allow ^
    protocol=TCP ^
    localport=8089 ^
    remoteip=127.0.0.1,::1 ^
    profile=any

if %errorlevel% eq 0 (
    echo [OK] Regra de firewall criada
) else (
    color 0C
    echo [ERRO] Falha ao criar regra de firewall
    pause
    exit /b 1
)

echo.
echo Verificando regra...
netsh advfirewall firewall show rule name="FC Data API Local" | findstr /i "habilitado ativo enabled"

echo.
color 0A
echo ========================================
echo    FIREWALL CONFIGURADO COM SUCESSO!
echo ========================================
echo.
echo A API só aceitará conexões locais (127.0.0.1)
echo.
echo Próximo passo: Execute 07_INICIAR_SERVICO.bat
echo.
pause