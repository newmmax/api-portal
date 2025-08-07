@echo off
echo ========================================
echo Testando Correções de Compilação
echo ========================================
echo.

cd /d "D:\PROJETOS\RUST\fc-data-api"

echo 1. Verificando sintaxe básica...
cargo check --message-format=short 2>&1

echo.
echo 2. Tentando compilação completa...
cargo build --message-format=short 2>&1

echo.
echo ========================================
echo Teste de correções finalizado
echo ========================================
