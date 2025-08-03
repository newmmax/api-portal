# 🚀 GUIA DE DEPLOY - FC DATA API

## 📋 Índice
1. [Preparação](#preparação)
2. [Build para Produção](#build-para-produção)
3. [Deploy Windows](#deploy-windows)
4. [Deploy Linux](#deploy-linux)
5. [Configuração Apache](#configuração-apache)
6. [Configuração Nginx](#configuração-nginx)
7. [Monitoramento](#monitoramento)
8. [Backup e Recuperação](#backup-e-recuperação)

## 🎯 Preparação

### 1. Configurar Ambiente de Produção

Crie um arquivo `.env.production`:

```env
# Produção
SERVER_HOST=0.0.0.0
SERVER_PORT=8089

# Bancos de Produção
DATABASE_URL=postgres://usuario:senha@servidor-prod:5432/fc_data
PORTAL_DATABASE_NAME=sys_pedidos
PORTAL_CONNECTION_STRING=Server=tcp:servidor-prod,1433;Database=sys_pedidos;UID=sa;PWD=senha_prod;TrustServerCertificate=true
PROTHEUS_DATABASE_NAME=sigaofc
PROTHEUS_CONNECTION_STRING=Server=tcp:servidor-prod,1433;Database=sigaofc;UID=sa;PWD=senha_prod;TrustServerCertificate=true

# JWT - MUDE ISSO!
JWT_SECRET=uma_chave_muito_segura_e_complexa_para_producao_2025
JWT_EXPIRATION_HOURS=24

# CORS - Domínios permitidos
CORS_ALLOWED_ORIGINS=https://app.suaempresa.com.br,https://portal.suaempresa.com.br

# Admin - MUDE ISSO!
ADMIN_USERNAME=admin_prod
ADMIN_PASSWORD=SenhaForteProducao2025!

# Logs
RUST_LOG=info
```

### 2. Checklist Pré-Deploy

- [ ] Alterar todas as senhas padrão
- [ ] Configurar JWT_SECRET único
- [ ] Testar conexões com bancos de produção
- [ ] Configurar CORS para domínios corretos
- [ ] Fazer backup das configurações
- [ ] Testar em ambiente de homologação

## 🔨 Build para Produção

### Windows

```powershell
# Na pasta do projeto
cd C:\XAMPP\htdocs\portaldepedidos\fc-data-api

# Build otimizado
cargo build --release

# Executável estará em:
# target\release\fc-data-api.exe
```

### Linux

```bash
# Na pasta do projeto
cd /opt/fc-data-api

# Build otimizado
cargo build --release

# Executável estará em:
# target/release/fc-data-api
```

## 🪟 Deploy Windows

### Opção 1: Windows Service com NSSM

1. **Baixar NSSM** (Non-Sucking Service Manager):
   - Download: https://nssm.cc/download
   - Extrair para `C:\nssm\`

2. **Preparar Diretórios**:
```powershell
# Criar estrutura
mkdir C:\fcdata-api
mkdir C:\fcdata-api\logs

# Copiar arquivos
copy target\release\fc-data-api.exe C:\fcdata-api\
copy .env.production C:\fcdata-api\.env
```

3. **Instalar como Serviço**:
```powershell
# Como Administrador
C:\nssm\nssm.exe install FCDataAPI

# Na interface gráfica:
# Path: C:\fcdata-api\fc-data-api.exe
# Startup directory: C:\fcdata-api
# Arguments: (deixe vazio)

# Aba Details:
# Display name: FC Data API Service
# Description: API unificada para Formula Certa

# Aba Log on:
# Log on as: Local System account

# Aba I/O:
# Output: C:\fcdata-api\logs\output.log
# Error: C:\fcdata-api\logs\error.log
```

4. **Gerenciar Serviço**:
```powershell
# Iniciar
nssm start FCDataAPI

# Parar
nssm stop FCDataAPI

# Reiniciar
nssm restart FCDataAPI

# Status
nssm status FCDataAPI

# Remover
nssm remove FCDataAPI
```

### Opção 2: Task Scheduler

1. Abrir Task Scheduler
2. Create Basic Task
3. Nome: "FC Data API"
4. Trigger: "When the computer starts"
5. Action: Start a program
6. Program: `C:\fcdata-api\fc-data-api.exe`
7. Start in: `C:\fcdata-api`
8. Marcar: "Run with highest privileges"

## 🐧 Deploy Linux

### 1. Preparar Ambiente

```bash
# Criar usuário
sudo useradd -r -s /bin/false fcdata

# Criar diretórios
sudo mkdir -p /opt/fc-data-api
sudo mkdir -p /var/log/fc-data-api

# Copiar arquivos
sudo cp target/release/fc-data-api /opt/fc-data-api/
sudo cp .env.production /opt/fc-data-api/.env

# Permissões
sudo chown -R fcdata:fcdata /opt/fc-data-api
sudo chown -R fcdata:fcdata /var/log/fc-data-api
sudo chmod +x /opt/fc-data-api/fc-data-api
```

### 2. Criar Serviço Systemd

```bash
# Criar arquivo de serviço
sudo nano /etc/systemd/system/fc-data-api.service
```

Conteúdo:
```ini
[Unit]
Description=FC Data API Service
After=network.target

[Service]
Type=simple
User=fcdata
Group=fcdata
WorkingDirectory=/opt/fc-data-api
Environment="RUST_LOG=info"
ExecStart=/opt/fc-data-api/fc-data-api
Restart=always
RestartSec=10

# Logs
StandardOutput=append:/var/log/fc-data-api/output.log
StandardError=append:/var/log/fc-data-api/error.log

# Segurança
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/log/fc-data-api

[Install]
WantedBy=multi-user.target
```

### 3. Gerenciar Serviço

```bash
# Recarregar configuração
sudo systemctl daemon-reload

# Habilitar início automático
sudo systemctl enable fc-data-api

# Iniciar
sudo systemctl start fc-data-api

# Status
sudo systemctl status fc-data-api

# Logs
sudo journalctl -u fc-data-api -f

# Parar
sudo systemctl stop fc-data-api

# Reiniciar
sudo systemctl restart fc-data-api
```

## 🌐 Configuração Apache

### Windows (XAMPP)

Edite `C:\xampp\apache\conf\extra\httpd-vhosts.conf`:

```apache
<VirtualHost *:443>
    ServerName api.suaempresa.com.br
    
    # SSL
    SSLEngine on
    SSLCertificateFile "C:/xampp/apache/conf/ssl.crt/server.crt"
    SSLCertificateKeyFile "C:/xampp/apache/conf/ssl.key/server.key"
    
    # Proxy para API
    ProxyPreserveHost On
    ProxyPass /services/api1 http://localhost:8089/services/api1
    ProxyPassReverse /services/api1 http://localhost:8089/services/api1
    
    # Headers
    Header always set Access-Control-Allow-Origin "*"
    Header always set Access-Control-Allow-Methods "GET, POST, PUT, DELETE, OPTIONS"
    Header always set Access-Control-Allow-Headers "Content-Type, Authorization"
    
    # Logs
    ErrorLog "logs/fc-data-api-error.log"
    CustomLog "logs/fc-data-api-access.log" common
</VirtualHost>
```

Habilite módulos:
```bash
# No httpd.conf, descomente:
LoadModule proxy_module modules/mod_proxy.so
LoadModule proxy_http_module modules/mod_proxy_http.so
LoadModule headers_module modules/mod_headers.so
LoadModule ssl_module modules/mod_ssl.so
```

### Linux

```bash
# Habilitar módulos
sudo a2enmod proxy proxy_http headers ssl

# Criar configuração
sudo nano /etc/apache2/sites-available/fc-data-api.conf
```

Conteúdo:
```apache
<VirtualHost *:443>
    ServerName api.suaempresa.com.br
    
    # SSL
    SSLEngine on
    SSLCertificateFile /etc/ssl/certs/seu-certificado.crt
    SSLCertificateKeyFile /etc/ssl/private/sua-chave.key
    
    # Proxy
    ProxyPreserveHost On
    ProxyPass /services/api1 http://localhost:8089/services/api1
    ProxyPassReverse /services/api1 http://localhost:8089/services/api1
    
    # Timeout
    ProxyTimeout 300
    
    # Headers de segurança
    Header always set X-Content-Type-Options "nosniff"
    Header always set X-Frame-Options "DENY"
    Header always set X-XSS-Protection "1; mode=block"
    
    # Logs
    ErrorLog ${APACHE_LOG_DIR}/fc-data-api-error.log
    CustomLog ${APACHE_LOG_DIR}/fc-data-api-access.log combined
</VirtualHost>
```

Ativar:
```bash
sudo a2ensite fc-data-api
sudo systemctl reload apache2
```

## 🔷 Configuração Nginx

```nginx
server {
    listen 443 ssl http2;
    server_name api.suaempresa.com.br;
    
    # SSL
    ssl_certificate /etc/nginx/ssl/cert.crt;
    ssl_certificate_key /etc/nginx/ssl/cert.key;
    
    # Segurança SSL
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;
    
    # Proxy para API
    location /services/api1 {
        proxy_pass http://localhost:8089;
        proxy_http_version 1.1;
        
        # Headers
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        
        # Timeout
        proxy_connect_timeout 300;
        proxy_send_timeout 300;
        proxy_read_timeout 300;
        
        # Buffering
        proxy_buffering off;
    }
    
    # Logs
    access_log /var/log/nginx/fc-data-api-access.log;
    error_log /var/log/nginx/fc-data-api-error.log;
}
```

## 📊 Monitoramento

### 1. Health Check Automatizado

Script para monitoramento (monitor.sh):
```bash
#!/bin/bash
HEALTH_URL="https://api.suaempresa.com.br/services/api1/health"
WEBHOOK_URL="https://hooks.slack.com/services/seu-webhook"

# Check health
RESPONSE=$(curl -s -o /dev/null -w "%{http_code}" $HEALTH_URL)

if [ $RESPONSE -ne 200 ]; then
    # Enviar alerta
    curl -X POST $WEBHOOK_URL \
        -H 'Content-Type: application/json' \
        -d '{"text":"⚠️ FC Data API está fora do ar!"}'
fi
```

Agendar no cron:
```bash
*/5 * * * * /opt/scripts/monitor.sh
```

### 2. Logs Rotation

Linux (`/etc/logrotate.d/fc-data-api`):
```
/var/log/fc-data-api/*.log {
    daily
    rotate 30
    compress
    delaycompress
    missingok
    notifempty
    create 0644 fcdata fcdata
    postrotate
        systemctl reload fc-data-api
    endscript
}
```

### 3. Métricas com Prometheus (Opcional)

Adicione ao seu código:
```rust
// Futuro: adicionar métricas
// use prometheus::{Counter, Histogram, Registry};
```

## 💾 Backup e Recuperação

### Script de Backup

```bash
#!/bin/bash
# backup-fc-api.sh

BACKUP_DIR="/backup/fc-data-api"
DATE=$(date +%Y%m%d_%H%M%S)

# Criar diretório
mkdir -p $BACKUP_DIR/$DATE

# Backup da aplicação
cp -r /opt/fc-data-api/* $BACKUP_DIR/$DATE/

# Backup de logs
tar -czf $BACKUP_DIR/$DATE/logs.tar.gz /var/log/fc-data-api/

# Limpar backups antigos (manter 30 dias)
find $BACKUP_DIR -type d -mtime +30 -exec rm -rf {} \;

echo "Backup completo em $BACKUP_DIR/$DATE"
```

### Recuperação

```bash
# Parar serviço
sudo systemctl stop fc-data-api

# Restaurar
cp -r /backup/fc-data-api/20250711_120000/* /opt/fc-data-api/

# Reiniciar
sudo systemctl start fc-data-api
```

## 🔒 Segurança em Produção

1. **Firewall**: Libere apenas porta 443 (HTTPS)
2. **SELinux/AppArmor**: Configure políticas
3. **Fail2ban**: Proteja contra brute force
4. **SSL**: Use certificados válidos (Let's Encrypt)
5. **Updates**: Mantenha sistema atualizado

## 🚨 Troubleshooting Produção

### API não inicia
```bash
# Verificar logs
journalctl -u fc-data-api -n 100

# Verificar permissões
ls -la /opt/fc-data-api/

# Testar manualmente
cd /opt/fc-data-api && ./fc-data-api
```

### Erro de conexão banco
- Verificar firewall
- Testar conectividade: `telnet servidor-db 1433`
- Verificar credenciais no .env

### Performance lenta
- Aumentar workers
- Verificar índices no banco
- Implementar cache (Redis)
- Usar CDN para assets

## 📞 Contatos de Emergência

Configure alertas para:
- Health check falhando
- Uso de CPU > 80%
- Uso de memória > 80%
- Erros 5xx frequentes
- Tempo de resposta > 2s
