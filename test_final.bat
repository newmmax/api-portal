@echo off
cd /d "D:\PROJETOS\RUST\fc-data-api"
echo Testando compilacao apos correcoes...
cargo check > compile_final.log 2>&1
if %ERRORLEVEL% == 0 (
    echo SUCESSO: Compilacao limpa!
    echo Output:
    type compile_final.log
) else (
    echo ERRO: Ainda ha problemas
    echo Output:
    type compile_final.log
)
