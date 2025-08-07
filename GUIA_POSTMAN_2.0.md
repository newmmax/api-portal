# 📋 GUIA DE USO - POSTMAN E DOCUMENTAÇÃO ATUALIZADA

## 🎯 Arquivos Atualizados (06/08/2025)

### 📦 Coleção Postman 2.0
- **Arquivo**: `FC_Data_API_2.0.postman_collection.json`
- **Endpoints**: 50+ organizados por módulos
- **Funcionalidades**: Auto-captura de token, testes automáticos, validações

### 🌍 Environments
- **Desenvolvimento**: `FC_Data_API_Dev.postman_environment.json`
- **Produção**: `FC_Data_API_Prod.postman_environment.json`

### 📚 Documentação
- **Arquivo**: `DOCUMENTACAO_API_2.0.md`
- **Conteúdo**: Documentação completa sincronizada com código atual

---

## 🚀 Como Importar no Postman

### 1. Importar Coleção
1. Abrir Postman
2. Clicar em **Import**
3. Selecionar `FC_Data_API_2.0.postman_collection.json`
4. Confirmar importação

### 2. Importar Environments
1. Clicar em **Import** novamente
2. Selecionar `FC_Data_API_Dev.postman_environment.json`
3. Repetir para `FC_Data_API_Prod.postman_environment.json`

### 3. Selecionar Environment
1. No canto superior direito, selecionar ambiente:
   - **FC Data API - DESENVOLVIMENTO** (para testes locais)
   - **FC Data API - PRODUÇÃO** (para testes em produção)

---

## 🔐 Primeiro Uso

### 1. Fazer Login
1. Selecionar pasta **🔐 Autenticação**
2. Executar request **Login**
3. ✅ Token será salvo automaticamente

### 2. Testar Endpoints
- Todos os outros endpoints usarão o token automaticamente
- Se token expirar (24h), executar Login novamente

---

## 📊 Módulos Disponíveis

### 📊 Data FC (PostgreSQL)
- **Vendas**: Query principal do sistema FC
- **Query Customizada**: SQL personalizado

### 🏢 Portal (SQL Server)
- **Franqueados**: Listar, buscar, CNPJ específico
- **Produtos**: Código, nome, query customizada

### ⚙️ Protheus (SQL Server)
- **Query**: SQL customizado no Protheus
- **Status Pedidos**: Status por número

### 📈 Analytics
- **Oportunidades**: Análise IA de pedidos
- **Recompra**: Análise de recompra inteligente
- **Exportações**: Relatórios em Excel/CSV

### 🛒 Pedidos
- **Geração IA**: Pedidos com sugestões inteligentes
- **CRUD**: Criar, buscar, atualizar, confirmar
- **Status**: Controle de status avançado
- **Tracking**: Sugestões aceitas/rejeitadas

### 🔍 Debug & Monitoring
- **Logs**: Visualizar e gerenciar logs
- **Debug Query**: Testar SQL sem autenticação

### ❤️ Health Check
- **Status**: Verificar saúde da API e bancos

---

## 🛠️ Funcionalidades Automáticas

### ✅ Auto-Captura de Token
- Login salva token automaticamente
- Todos os endpoints usam token salvo
- Renovação automática sugerida quando expira

### 🧪 Testes Integrados
- Validação de status codes
- Verificação de estrutura de resposta
- Logs automáticos de sucesso/erro

### 🔄 Scripts Pre/Post Request
- Datas dinâmicas atualizadas automaticamente
- Verificação de tokens antes de requests
- Logs detalhados de debug

---

## 🌐 URLs por Ambiente

### Desenvolvimento
```
URL: http://localhost:8089/services/api1
Uso: Desenvolvimento local
Debug: Habilitado
```

### Produção
```
URL: https://conexao.artesanalfarmacia.com.br/services/api1
Uso: Sistema em produção
SSL: Verificação habilitada
```

---

## 📝 Variáveis Importantes

### Desenvolvimento
- `base_url`: URL completa da API
- `token`: JWT salvo automaticamente
- `test_cnpj`: CNPJ para testes
- `test_produto_codigo`: Código produto teste
- `debug_mode`: true

### Produção
- `base_url`: URL produção HTTPS
- `token`: JWT produção
- `ssl_verify`: true
- `timeout_ms`: 60000 (maior)
- Variáveis de teste **desabilitadas** (usar dados reais)

---

## 🚨 Dicas Importantes

### ✅ Boas Práticas
- **Sempre fazer Login primeiro** antes de testar outros endpoints
- **Usar ambiente correto**: Dev para testes, Prod para validação
- **Verificar token expirado**: Se 401, fazer novo Login
- **Dados reais em produção**: Não usar valores de teste

### ⚠️ Cuidados em Produção
- **Usar dados reais**: CNPJs, códigos, IDs válidos
- **Timeouts maiores**: Produção pode ser mais lenta
- **SSL ativo**: Certificados devem ser válidos
- **Logs limitados**: Debug mode desabilitado

---

## 📞 Suporte

### Problemas Comuns
1. **Token inválido**: Executar Login novamente
2. **Ambiente errado**: Verificar seleção no canto superior direito
3. **URL incorreta**: Verificar variável `base_url` no environment
4. **Permissões**: Verificar se API está rodando na porta correta

### Recursos
- **Documentação completa**: `DOCUMENTACAO_API_2.0.md`
- **Contexto do projeto**: Arquivos `CONTEXTO_*.md`
- **Scripts de deploy**: Pasta `temp_deploy/`

---

✅ **Tudo pronto para uso!** A coleção e documentação estão 100% sincronizadas com o código atual do FC Data API 2.0.