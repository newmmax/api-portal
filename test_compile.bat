@echo off
echo 🚀 Testando compilação do FC Data API...
cd /d "D:\PROJETOS\RUST\fc-data-api"
cargo check
if %ERRORLEVEL% == 0 (
    echo ✅ SUCESSO: Compilação sem erros!
) else (
    echo ❌ ERRO: Ainda há problemas de compilação
)
pause
