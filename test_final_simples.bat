@echo off
echo Testando compilacao final...
cd /d "D:\PROJETOS\RUST\fc-data-api"
cargo check > test_final_resultado.txt 2>&1
if %ERRORLEVEL% == 0 (
    echo SUCESSO! Compilacao limpa.
) else (
    echo ERRO encontrado.
)
echo Resultado salvo em test_final_resultado.txt
