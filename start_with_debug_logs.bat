@echo off
REM Script para ativar logs DEBUG temporariamente
echo 🔍 ATIVANDO LOGS DEBUG - FC DATA API

REM Definir variáveis de ambiente para esta sessão
set RUST_LOG=debug,fc_data_api=debug,actix_web=debug,sqlx=debug
set ENABLE_DEBUG_LOGS=true
set DEBUG_LOG_FILE=cards_debug.log

echo ✅ Logs DEBUG ativados para esta sessão:
echo    - RUST_LOG: %RUST_LOG%
echo    - ENABLE_DEBUG_LOGS: %ENABLE_DEBUG_LOGS%
echo    - DEBUG_LOG_FILE: %DEBUG_LOG_FILE%
echo.

REM Navegar para pasta do projeto
cd /d "D:\PROJETOS\RUST\fc-data-api"

echo 🚀 Iniciando FC Data API com logs DEBUG...
echo 📋 Pressione Ctrl+C para parar
echo.

REM Executar a API
cargo run