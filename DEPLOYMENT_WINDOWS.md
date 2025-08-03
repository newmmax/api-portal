# 🚀 Guia de Implantação Windows - FC Data API

## 📋 Visão Geral

Este guia detalha como implantar a FC Data API como um serviço Windows robusto que:
- ✅ Inicia automaticamente com o Windows
- ✅ Reinicia em caso de falha
- ✅ Monitora e registra logs
- ✅ Recupera de crashes
- ✅ Sobrevive a reinicializações

## 🛠️ Pré-requisitos

1. **Windows Server 2012+ ou Windows 10/11**
2. **NSSM** (Non-Sucking Service Manager)
   - Download: https://nssm.cc/download
   - Extraia para `C:\nssm\` ou adicione ao PATH
3. **Executável compilado** da API
4. **Acesso administrativo** ao servidor

## 📦 Passo 1: Preparar Arquivos

### 1.1 Compilar Release
```bash
cd C:\XAMPP\htdocs\portaldepedidos\fc-data-api
cargo build --release
```

### 1.2 Criar Estrutura de Diretórios
```cmd
mkdir C:\fcdata-api
mkdir C:\fcdata-api\logs
mkdir C:\fcdata-api\backup
```

### 1.3 Copiar Arquivos
```cmd
copy target\release\fc-data-api.exe C:\fcdata-api\
copy .env C:\fcdata-api\
copy README.md C:\fcdata-api\
```

### 1.4 Ajustar .env para Produção
Edite `C:\fcdata-api\.env`:
```env
# Configurações de produção
SERVER_HOST=0.0.0.0  # Escutar em todas as interfaces
SERVER_PORT=8080
RUST_LOG=info,fc_data_api=info  # Logs moderados
```

## 🔧 Passo 2: Instalar como Serviço

### 2.1 Executar Script de Instalação

**Como Administrador**, execute:
```cmd
cd C:\XAMPP\htdocs\portaldepedidos\fc-data-api
install_service.bat
```

### 2.2 Ou Instalação Manual com NSSM

```cmd
# Instalar serviço
nssm install FCDataAPI "C:\fcdata-api\fc-data-api.exe"

# Configurações básicas
nssm set FCDataAPI AppDirectory "C:\fcdata-api"
nssm set FCDataAPI DisplayName "FC Data API Service"
nssm set FCDataAPI Description "API REST para consulta de dados FC PostgreSQL"
nssm set FCDataAPI Start SERVICE_AUTO_START

# Configurar recuperação de falhas
nssm set FCDataAPI AppThrottle 1500
nssm set FCDataAPI AppExit Default Restart
nssm set FCDataAPI AppRestartDelay 5000

# Configurar logs
nssm set FCDataAPI AppStdout "C:\fcdata-api\logs\service.log"
nssm set FCDataAPI AppStderr "C:\fcdata-api\logs\error.log"
nssm set FCDataAPI AppRotateFiles 1
nssm set FCDataAPI AppRotateOnline 1
nssm set FCDataAPI AppRotateBytes 10485760

# Iniciar serviço
nssm start FCDataAPI
```

## 🛡️ Passo 3: Configurar Recuperação Avançada

### 3.1 Via Serviços do Windows
1. Abra `services.msc`
2. Encontre "FC Data API Service"
3. Propriedades → Aba "Recuperação"
4. Configure:
   - **Primeira falha**: Reiniciar o Serviço
   - **Segunda falha**: Reiniciar o Serviço
   - **Falhas subsequentes**: Reiniciar o Serviço
   - **Reiniciar serviço após**: 1 minuto
   - **Redefinir contador após**: 1 dia

### 3.2 Via PowerShell (Automático)
```powershell
# Executar como Administrador
$serviceName = "FCDataAPI"
sc.exe failure $serviceName reset= 86400 actions= restart/60000/restart/60000/restart/60000
```

## 📊 Passo 4: Monitoramento e Logs

### 4.1 Configurar Rotação de Logs
Crie `C:\fcdata-api\rotate_logs.bat`:
```batch
@echo off
set LOGDIR=C:\fcdata-api\logs
set BACKUPDIR=C:\fcdata-api\backup

REM Criar backup com timestamp
for /f "tokens=2 delims==" %%a in ('wmic OS Get localdatetime /value') do set "dt=%%a"
set "timestamp=%dt:~0,4%-%dt:~4,2%-%dt:~6,2%_%dt:~8,2%-%dt:~10,2%"

REM Mover logs antigos
move "%LOGDIR%\service.log" "%BACKUPDIR%\service_%timestamp%.log" 2>nul
move "%LOGDIR%\error.log" "%BACKUPDIR%\error_%timestamp%.log" 2>nul

REM Limpar backups antigos (manter últimos 30 dias)
forfiles /p "%BACKUPDIR%" /m *.log /d -30 /c "cmd /c del @path" 2>nul
```

### 4.2 Agendar Rotação de Logs
```cmd
# Criar tarefa agendada para rotação semanal
schtasks /create /tn "FCDataAPI_LogRotation" /tr "C:\fcdata-api\rotate_logs.bat" /sc weekly /d SUN /st 00:00 /ru SYSTEM
```

## 🔍 Passo 5: Monitoramento de Saúde

### 5.1 Script de Health Check
Crie `C:\fcdata-api\health_check.ps1`:
```powershell
# Health Check Script
$url = "http://localhost:8080/services/api1/health"
$logFile = "C:\fcdata-api\logs\health_check.log"

try {
    $response = Invoke-WebRequest -Uri $url -TimeoutSec 10
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    
    if ($response.StatusCode -eq 200) {
        "$timestamp - Health check OK" | Out-File $logFile -Append
    } else {
        "$timestamp - Health check failed: $($response.StatusCode)" | Out-File $logFile -Append
        # Reiniciar serviço se falhar
        Restart-Service FCDataAPI
    }
} catch {
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    "$timestamp - Health check error: $_" | Out-File $logFile -Append
    # Reiniciar serviço em caso de erro
    Restart-Service FCDataAPI
}
```

### 5.2 Agendar Health Check
```cmd
# Verificar saúde a cada 5 minutos
schtasks /create /tn "FCDataAPI_HealthCheck" /tr "powershell.exe -ExecutionPolicy Bypass -File C:\fcdata-api\health_check.ps1" /sc minute /mo 5 /ru SYSTEM
```

## 🔐 Passo 6: Segurança

### 6.1 Configurar Firewall
```powershell
# Adicionar regra de firewall (executar como Admin)
New-NetFirewallRule -DisplayName "FC Data API" -Direction Inbound -LocalPort 8080 -Protocol TCP -Action Allow
```

### 6.2 Permissões de Pasta
```cmd
# Definir permissões apropriadas
icacls "C:\fcdata-api" /grant "NT SERVICE\FCDataAPI:(OI)(CI)F" /T
icacls "C:\fcdata-api\logs" /grant "NT SERVICE\FCDataAPI:(OI)(CI)F" /T
```

## 🔄 Passo 7: Backup e Recuperação

### 7.1 Script de Backup
Crie `C:\fcdata-api\backup_config.bat`:
```batch
@echo off
set SRCDIR=C:\fcdata-api
set BACKUPDIR=D:\Backups\FCDataAPI

REM Criar backup com data
for /f "tokens=2 delims==" %%a in ('wmic OS Get localdatetime /value') do set "dt=%%a"
set "date=%dt:~0,4%-%dt:~4,2%-%dt:~6,2%"

REM Criar diretório de backup
mkdir "%BACKUPDIR%\%date%" 2>nul

REM Copiar arquivos importantes
copy "%SRCDIR%\.env" "%BACKUPDIR%\%date%\" /Y
copy "%SRCDIR%\*.exe" "%BACKUPDIR%\%date%\" /Y
xcopy "%SRCDIR%\logs" "%BACKUPDIR%\%date%\logs\" /E /I /Y

echo Backup concluído em %BACKUPDIR%\%date%
```

### 7.2 Agendar Backup Diário
```cmd
schtasks /create /tn "FCDataAPI_Backup" /tr "C:\fcdata-api\backup_config.bat" /sc daily /st 02:00 /ru SYSTEM
```

## 📈 Passo 8: Performance e Otimização

### 8.1 Configurar Prioridade do Processo
```cmd
# Definir prioridade alta para o serviço
nssm set FCDataAPI AppPriority ABOVE_NORMAL_PRIORITY_CLASS
```

### 8.2 Limitar Uso de Memória (Opcional)
```cmd
# Limitar a 2GB de RAM
nssm set FCDataAPI AppMemoryLimit 2097152
```

## 🚨 Troubleshooting

### Serviço não inicia
1. Verificar logs em `C:\fcdata-api\logs\`
2. Testar executável manualmente:
   ```cmd
   cd C:\fcdata-api
   fc-data-api.exe
   ```

### Porta em uso
```cmd
# Verificar qual processo usa a porta
netstat -ano | findstr :8080
# Matar processo se necessário
taskkill /PID [PID_NUMBER] /F
```

### Verificar status do serviço
```cmd
# Status detalhado
sc query FCDataAPI
nssm status FCDataAPI

# Logs do serviço
eventvwr.msc
# Procurar em: Applications and Services Logs
```

## 📱 Comandos Úteis

```cmd
# Parar serviço
nssm stop FCDataAPI

# Iniciar serviço
nssm start FCDataAPI

# Reiniciar serviço
nssm restart FCDataAPI

# Remover serviço
nssm remove FCDataAPI confirm

# Editar configurações
nssm edit FCDataAPI
```

## 🔔 Notificações (Opcional)

### Email em caso de falha
Crie `C:\fcdata-api\send_alert.ps1`:
```powershell
param($Subject, $Body)

$smtp = "smtp.gmail.com"
$port = 587
$from = "alertas@empresa.com"
$to = "admin@empresa.com"
$password = "senha_app"

$message = New-Object System.Net.Mail.MailMessage
$message.From = $from
$message.To.Add($to)
$message.Subject = $Subject
$message.Body = $Body

$smtp = New-Object System.Net.Mail.SmtpClient($smtp, $port)
$smtp.EnableSSL = $true
$smtp.Credentials = New-Object System.Net.NetworkCredential($from, $password)
$smtp.Send($message)
```

Integrar no health check para alertas automáticos.

## ✅ Checklist Final

- [ ] Executável compilado em modo release
- [ ] Arquivos copiados para C:\fcdata-api
- [ ] .env configurado para produção
- [ ] Serviço instalado com NSSM
- [ ] Recuperação de falhas configurada
- [ ] Logs configurados e com rotação
- [ ] Health check agendado
- [ ] Firewall configurado
- [ ] Backup agendado
- [ ] Apache proxy configurado
- [ ] Testado acesso via HTTPS

## 📞 Suporte

Em caso de problemas:
1. Verificar logs em `C:\fcdata-api\logs\`
2. Executar `check_config.bat` para diagnóstico
3. Revisar Event Viewer do Windows
4. Testar manualmente antes de criar serviço
