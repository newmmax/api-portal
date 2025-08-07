@echo off
REM Script para visualizar logs em tempo real
echo 🔍 VISUALIZADOR DE LOGS - FC DATA API
echo.

cd /d "D:\PROJETOS\RUST\fc-data-api"

:MENU
echo ============================================
echo  ESCOLHA O TIPO DE LOG PARA VISUALIZAR:
echo ============================================
echo.
echo 1) 📊 Logs Cards Analytics (cards_debug.log)
echo 2) 🚀 Iniciar API com logs DEBUG no console
echo 3) 📋 Ver últimas 50 linhas dos Cards
echo 4) 🔄 Monitorar Cards em tempo real
echo 5) 🧹 Limpar arquivo de logs Cards
echo 6) ❌ Sair
echo.
set /p choice="Digite sua escolha (1-6): "

if "%choice%"=="1" goto SHOW_CARDS_LOG
if "%choice%"=="2" goto START_DEBUG
if "%choice%"=="3" goto TAIL_CARDS
if "%choice%"=="4" goto MONITOR_CARDS
if "%choice%"=="5" goto CLEAR_LOGS
if "%choice%"=="6" goto END

echo ❌ Opção inválida!
goto MENU

:SHOW_CARDS_LOG
echo 📋 Mostrando conteúdo completo do cards_debug.log:
echo ================================================
if exist cards_debug.log (
    type cards_debug.log
) else (
    echo ⚠️ Arquivo cards_debug.log não encontrado
)
echo.
pause
goto MENU

:START_DEBUG
echo 🚀 Iniciando API com logs DEBUG no console...
set RUST_LOG=debug,fc_data_api=debug,actix_web=debug
set ENABLE_DEBUG_LOGS=true
cargo run
goto MENU

:TAIL_CARDS
echo 📋 Últimas 50 linhas do cards_debug.log:
echo =======================================
if exist cards_debug.log (
    powershell "Get-Content cards_debug.log | Select-Object -Last 50"
) else (
    echo ⚠️ Arquivo cards_debug.log não encontrado
)
echo.
pause
goto MENU

:MONITOR_CARDS
echo 🔄 Monitorando cards_debug.log em tempo real...
echo ============================================
echo Pressione Ctrl+C para parar
echo.
if exist cards_debug.log (
    powershell "Get-Content cards_debug.log -Wait"
) else (
    echo ⚠️ Arquivo cards_debug.log não encontrado
    echo Execute a API primeiro para gerar logs
)
pause
goto MENU

:CLEAR_LOGS
echo 🧹 Limpando arquivo cards_debug.log...
if exist cards_debug.log (
    del cards_debug.log
    echo ✅ Arquivo cards_debug.log removido
) else (
    echo ⚠️ Arquivo não encontrado
)
pause
goto MENU

:END
echo 👋 Saindo...
exit /b 0