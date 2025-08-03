@echo off
echo ===================================
echo FC Data API - Teste Local
echo ===================================
echo.

echo Verificando configuracao...
findstr "SERVER_PORT" .env
echo.

echo Iniciando API em modo debug...
echo URL padrao: http://localhost:8080/services/api1
echo (Se mudou a porta no .env, use a porta configurada)
echo.

cd /d "C:\XAMPP\htdocs\portaldepedidos\fc-data-api"
target\debug\fc-data-api.exe

pause
