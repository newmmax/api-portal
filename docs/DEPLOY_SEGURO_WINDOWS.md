# 🛡️ GUIA DE DEPLOY SEGURO - FC DATA API NO WINDOWS

## ⚠️ CHECKLIST DE SEGURANÇA PRÉ-DEPLOY

### 1. BACKUPS OBRIGATÓRIOS
- [ ] Backup do Portal de Pedidos funcionando
- [ ] Backup dos bancos de dados
- [ ] Anotar configurações atuais do sistema
- [ ] Ter acesso remoto alternativo (TeamViewer/AnyDesk)

### 2. INFORMAÇÕES NECESSÁRIAS
- [ ] Senha de administrador do Windows
- [ ] Credenciais dos bancos de dados
- [ ] Portas disponíveis no servidor
- [ ] Configuração atual do Apache/IIS

### 3. PLANO DE ROLLBACK
- [ ] Como reverter cada mudança
- [ ] Contatos de emergência
- [ ] Horário de menor movimento

---

## 📁 PASSO 1: CRIAR ESTRUTURA SEGURA

### 1.1 Criar Diretórios
```batch
REM Execute como Administrador
mkdir C:\fcdata-api
mkdir C:\fcdata-api\backup
mkdir C:\fcdata-api\logs
mkdir C:\fcdata-api\app
```

### 1.2 Verificar Estrutura
```batch
dir C:\fcdata-api
```

Resultado esperado:
```
C:\fcdata-api
├── app\       (onde ficará a API)
├── backup\    (backups de configuração)
└── logs\      (logs da aplicação)
```

---

## 🔍 PASSO 2: VERIFICAR PORTAS DISPONÍVEIS

### 2.1 Listar Portas em Uso
```batch
netstat -an | findstr LISTENING | findstr :80
```

### 2.2 Escolher Porta para API
Sugestões de portas seguras:
- 8089 (padrão da API)
- 8090 (alternativa)
- 9000 (alternativa)

### 2.3 Testar Porta Escolhida
```batch
REM Substitua 8089 pela porta escolhida
netstat -an | findstr :8089
```

Se não retornar nada, a porta está livre ✅

---

## 🔒 PASSO 3: CONFIGURAR FIREWALL

### 3.1 Adicionar Regra de Entrada (APENAS LOCAL)
```batch
REM IMPORTANTE: Apenas acesso local, não externo!
netsh advfirewall firewall add rule name="FC Data API Local" dir=in action=allow protocol=TCP localport=8089 remoteip=127.0.0.1,::1
```

### 3.2 Verificar Regra Criada
```batch
netsh advfirewall firewall show rule name="FC Data API Local"
```

---

## 📦 PASSO 4: PREPARAR APLICAÇÃO

### 4.1 Compilar em Modo Release
No seu PC de desenvolvimento:
```batch
cd C:\XAMPP\htdocs\portaldepedidos\fc-data-api
cargo build --release
```

### 4.2 Arquivos Necessários
Copie estes arquivos para um pendrive/pasta:
```
- target\release\fc-data-api.exe
- .env.production (crie baseado no .env)
```

### 4.3 Criar .env.production SEGURO
```env
# PRODUÇÃO - PORTAL DE PEDIDOS
SERVER_HOST=127.0.0.1
SERVER_PORT=8089

# PostgreSQL - Formula Certa (não mude)
DATABASE_URL=postgres://rodrigo:R0drigoPgSQL@10.216.1.16:5432/fc_data

# SQL Server - Portal de Pedidos PRODUÇÃO
PORTAL_DATABASE_HOST=localhost
PORTAL_DATABASE_PORT=1433
PORTAL_DATABASE_NAME=sys_pedidos
PORTAL_DATABASE_USER=sa
PORTAL_DATABASE_PASS=5y54dm1n%
PORTAL_CONNECTION_STRING=Server=tcp:localhost,1433;Database=sys_pedidos;UID=sa;PWD=5y54dm1n%;TrustServerCertificate=true

# SQL Server - Protheus PRODUÇÃO
PROTHEUS_DATABASE_HOST=localhost
PROTHEUS_DATABASE_PORT=1433
PROTHEUS_DATABASE_NAME=sigaofc
PROTHEUS_DATABASE_USER=sa
PROTHEUS_DATABASE_PASS=5y54dm1n%
PROTHEUS_CONNECTION_STRING=Server=tcp:localhost,1433;Database=sigaofc;UID=sa;PWD=5y54dm1n%;TrustServerCertificate=true

# JWT - MUDE ISSO ANTES DO DEPLOY!
JWT_SECRET=fc_data_api_producao_2025_chave_segura_artesanal
JWT_EXPIRATION_HOURS=24

# CORS - Apenas origens confiáveis
CORS_ALLOWED_ORIGINS=http://localhost,http://127.0.0.1

# Admin - MUDE ISSO!
ADMIN_USERNAME=admin_prod
ADMIN_PASSWORD=Pr0duc@0_FC_2025!

# Logs
RUST_LOG=info

# API
API_PREFIX=/services/api1
```

---

## 🚀 PASSO 5: DEPLOY MANUAL (TESTE PRIMEIRO)

### 5.1 Copiar Arquivos
```batch
REM No servidor, como Administrador
copy E:\fc-data-api.exe C:\fcdata-api\app\
copy E:\.env.production C:\fcdata-api\app\.env
```

### 5.2 Teste Manual PRIMEIRO
```batch
cd C:\fcdata-api\app
fc-data-api.exe
```

Deve aparecer:
```
[INFO] 🚀 Iniciando FC Data API...
[INFO] 📊 Conectando aos bancos de dados...
[INFO] ✅ Pools de conexões criados com sucesso
[INFO] 🌐 Servidor rodando em http://127.0.0.1:8089
```

### 5.3 Testar Health Check
Abra outro prompt:
```batch
curl http://127.0.0.1:8089/services/api1/health
```

Se funcionar, pressione Ctrl+C no primeiro prompt para parar.

---

## 🔨 PASSO 6: INSTALAR COMO SERVIÇO

### 6.1 Baixar NSSM
1. Baixe de: https://nssm.cc/download
2. Extraia nssm.exe para C:\fcdata-api\tools\

### 6.2 Instalar Serviço
```batch
cd C:\fcdata-api\tools
nssm install FCDataAPI
```

Na janela que abrir:
- **Path**: C:\fcdata-api\app\fc-data-api.exe
- **Startup directory**: C:\fcdata-api\app
- **Arguments**: (deixe vazio)

Aba **Details**:
- **Display name**: FC Data API
- **Description**: API unificada Formula Certa

Aba **Log on**:
- **Log on as**: Local System account

Aba **I/O**:
- **Output**: C:\fcdata-api\logs\output.log
- **Error**: C:\fcdata-api\logs\error.log

Aba **File rotation**:
- **Replace existing Output and Error files**: Marcado
- **Rotate files**: Marcado
- **Rotate while service is running**: Marcado
- **File size**: 10 MB

Clique em **Install service**

### 6.3 NÃO INICIE AINDA!
```batch
REM Verifique se foi criado
nssm status FCDataAPI
```

---

## 🌐 PASSO 7: CONFIGURAR PROXY REVERSO

### 7.1 Backup da Configuração Apache
```batch
cd C:\xampp\apache\conf
copy httpd.conf httpd.conf.backup
cd extra
copy httpd-vhosts.conf httpd-vhosts.conf.backup
```

### 7.2 Habilitar Módulos no httpd.conf
Encontre e descomente estas linhas:
```apache
LoadModule proxy_module modules/mod_proxy.so
LoadModule proxy_http_module modules/mod_proxy_http.so
LoadModule headers_module modules/mod_headers.so
```

### 7.3 Adicionar Virtual Host
Em `httpd-vhosts.conf`, adicione NO FINAL:
```apache
# FC Data API Proxy
<Location /services/api1>
    ProxyPreserveHost On
    ProxyPass http://127.0.0.1:8089/services/api1
    ProxyPassReverse http://127.0.0.1:8089/services/api1
    
    # Timeout
    ProxyTimeout 300
    
    # Headers
    RequestHeader set X-Forwarded-Proto "http"
    RequestHeader set X-Forwarded-For "%{REMOTE_ADDR}s"
</Location>
```

### 7.4 Testar Configuração Apache
```batch
cd C:\xampp\apache\bin
httpd -t
```

Deve retornar: "Syntax OK"

---

## ✅ PASSO 8: ATIVAÇÃO GRADUAL

### 8.1 Iniciar Serviço
```batch
nssm start FCDataAPI
```

### 8.2 Verificar Logs
```batch
type C:\fcdata-api\logs\output.log
```

### 8.3 Testar Diretamente
```batch
curl http://127.0.0.1:8089/services/api1/health
```

### 8.4 Reiniciar Apache
```batch
C:\xampp\apache\bin\httpd -k restart
```

### 8.5 Testar Via Apache
```batch
curl http://localhost/services/api1/health
```

---

## 🔄 PASSO 9: MONITORAMENTO

### 9.1 Criar Script de Monitoramento
Crie `C:\fcdata-api\monitor.bat`:
```batch
@echo off
:loop
curl -s http://127.0.0.1:8089/services/api1/health > nul
if %errorlevel% neq 0 (
    echo %date% %time% - API DOWN >> C:\fcdata-api\logs\monitor.log
    nssm restart FCDataAPI
)
timeout /t 60 > nul
goto loop
```

### 9.2 Adicionar ao Agendador de Tarefas
1. Abrir Task Scheduler
2. Create Basic Task
3. Nome: "FC API Monitor"
4. Trigger: "When computer starts"
5. Action: Start a program
6. Program: C:\fcdata-api\monitor.bat
7. Start in: C:\fcdata-api

---

## 🚨 PASSO 10: PLANO DE ROLLBACK

### Se algo der errado:

#### 10.1 Parar Serviço
```batch
nssm stop FCDataAPI
```

#### 10.2 Reverter Apache
```batch
cd C:\xampp\apache\conf
copy httpd.conf.backup httpd.conf
cd extra
copy httpd-vhosts.conf.backup httpd-vhosts.conf
C:\xampp\apache\bin\httpd -k restart
```

#### 10.3 Remover Serviço
```batch
nssm remove FCDataAPI confirm
```

#### 10.4 Remover Regra Firewall
```batch
netsh advfirewall firewall delete rule name="FC Data API Local"
```

---

## 📝 TESTES FINAIS

### 1. Login
```batch
curl -X POST http://localhost/services/api1/auth/login -H "Content-Type: application/json" -d "{\"username\": \"admin_prod\", \"password\": \"Pr0duc@0_FC_2025!\"}"
```

### 2. Verificar Integração Portal
No navegador, acesse o Portal e verifique se continua funcionando normalmente.

### 3. Logs
```batch
REM Ver últimas 20 linhas do log
powershell -command "& {Get-Content C:\fcdata-api\logs\output.log -Tail 20}"
```

---

## 🎯 COMANDOS ÚTEIS

### Status do Serviço
```batch
nssm status FCDataAPI
```

### Reiniciar Serviço
```batch
nssm restart FCDataAPI
```

### Ver Configuração
```batch
nssm dump FCDataAPI
```

### Logs em Tempo Real
```batch
powershell -command "& {Get-Content C:\fcdata-api\logs\output.log -Wait}"
```

---

## ⚠️ IMPORTANTE

1. **NUNCA** exponha a porta 8089 para internet
2. **SEMPRE** faça backup antes de qualquer mudança
3. **TESTE** cada etapa antes de prosseguir
4. **MONITORE** os logs após o deploy
5. **DOCUMENTE** qualquer mudança feita

---

## 📞 EM CASO DE EMERGÊNCIA

1. Pare o serviço: `nssm stop FCDataAPI`
2. Reverta Apache com os backups
3. Verifique logs em `C:\fcdata-api\logs\`
4. Portal continua funcionando mesmo sem a API

Lembre-se: O Portal funciona independentemente da API!
