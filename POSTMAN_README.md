# 📮 Postman Collection - FC Data API

## 📥 Como Importar no Postman

1. **Abra o Postman**

2. **Importe a Coleção:**
   - Clique em **Import** (botão no canto superior esquerdo)
   - Arraste o arquivo `FC_Data_API.postman_collection.json`
   - Ou clique em "Upload Files" e selecione o arquivo

3. **Importe os Ambientes:**
   - Repita o processo para:
     - `FC_Data_API_Dev.postman_environment.json` (Desenvolvimento)
     - `FC_Data_API_Prod.postman_environment.json` (Produção)

4. **Selecione o Ambiente:**
   - No canto superior direito, selecione o ambiente desejado:
     - **FC Data API - Desenvolvimento** para testes locais
     - **FC Data API - Produção** para API em produção

## 🚀 Como Usar

### 1. Primeiro Acesso
1. Selecione o ambiente de **Desenvolvimento**
2. Execute **Health Check** para verificar se a API está rodando
3. Execute **Login** para obter o token JWT
   - O token será salvo automaticamente nas variáveis

### 2. Fazendo Consultas
- Após o login, todas as requisições autenticadas funcionarão automaticamente
- O token é incluído no header `Authorization: Bearer {{token}}`

### 3. Ordem Recomendada de Testes
1. **Health Check** - Verificar conexão
2. **Login** - Obter token
3. **Validar Token** - Confirmar autenticação
4. **Vendas Resumidas** - Teste básico
5. **Vendas Detalhadas** - Teste completo
6. **Query Customizada** - Queries SQL personalizadas

## 📋 Estrutura da Coleção

```
FC Data API/
├── Health Check (sem autenticação)
├── Autenticação/
│   ├── Login
│   └── Validar Token
├── Consultas de Vendas/
│   ├── Vendas Resumidas
│   ├── Vendas Detalhadas
│   └── Query Customizada
└── Exemplos de Queries/
    ├── Total de Vendas por Empresa
    ├── Top 10 Produtos Mais Vendidos
    └── Vendas por Vendedor
```

## 🔧 Variáveis Disponíveis

### Variáveis de Coleção
- `{{protocol}}` - Protocolo (http/https)
- `{{host}}` - Host/domínio
- `{{port}}` - Porta do servidor (facilmente configurável)
- `{{api_path}}` - Caminho da API (/services/api1)
- `{{base_url}}` - URL completa montada automaticamente
- `{{token}}` - Token JWT (preenchido automaticamente após login)

### Variáveis de Ambiente
- **Desenvolvimento:**
  - `protocol`: http
  - `host`: localhost
  - `port`: 8080 (⚠️ MUDE AQUI SE NECESSÁRIO)
  - `api_path`: /services/api1
  - `username`: admin
  - `password`: ArtesanalFC2025!

- **Produção:**
  - `protocol`: https
  - `host`: conexao.artesanalfarmacia.com.br
  - `port`: 443 (padrão HTTPS)
  - Mesmas credenciais

### 🔄 Como Mudar a Porta

Se a API estiver rodando em outra porta:

1. **No Postman:**
   - Vá em **Environments** (ícone de engrenagem)
   - Selecione o ambiente (Dev ou Prod)
   - Mude o valor da variável `port`
   - Salve com Ctrl+S

2. **Exemplo:** API rodando na porta 8089
   - Mude `port` de `8080` para `8089`
   - Todas as requisições usarão automaticamente a nova porta

## 📝 Filtros Disponíveis

### Vendas Resumidas e Detalhadas
- `data_inicio` - Data inicial (YYYY-MM-DD)
- `data_fim` - Data final (YYYY-MM-DD)
- `limite` - Número máximo de registros
- `empresa` - Nome da empresa
- `filial` - Código da filial
- `vendedor` - Código do vendedor (apenas detalhadas)
- `produto` - Nome do produto (apenas detalhadas, busca parcial)

### Query Customizada
Aceita qualquer query SELECT válida. Exemplos incluídos:
- Total de vendas por empresa
- Top 10 produtos mais vendidos
- Performance por vendedor

## 🧪 Testes Automatizados

Cada requisição inclui testes automatizados que verificam:
- Status HTTP correto (200)
- Estrutura da resposta
- Presença de campos obrigatórios
- Salvamento automático do token

## 🔒 Segurança

- Token JWT válido por 24 horas
- Apenas queries SELECT são permitidas
- Parâmetros preparados para evitar SQL injection
- CORS configurado para domínios autorizados

## 💡 Dicas

1. **Token Expirado?**
   - Execute o endpoint de Login novamente

2. **Erro 401?**
   - Token inválido ou expirado
   - Faça login novamente

3. **Queries Customizadas:**
   - Use sempre aspas duplas para strings SQL
   - Teste primeiro queries simples
   - Evite queries muito complexas que possam timeout

4. **Performance:**
   - Use sempre o parâmetro `limite` para grandes conjuntos de dados
   - Filtros de data melhoram significativamente a performance

## 📊 Exemplos de Uso

### Buscar vendas do último mês:
```json
{
  "data_inicio": "2025-06-01",
  "data_fim": "2025-06-30",
  "limite": 100
}
```

### Query para análise de produtos:
```sql
SELECT 
    p.setor,
    COUNT(DISTINCT v.cdpro) as produtos,
    SUM(v.vrrcb) as faturamento
FROM fc14100 v
INNER JOIN fc03000 p ON v.cdpro = p.cdpro
WHERE v.dtsai >= '2025-01-01'
GROUP BY p.setor
ORDER BY faturamento DESC
```

## 🆘 Suporte

- Logs da API: `C:\fcdata-api\logs\`
- Configuração: `C:\fcdata-api\.env`
- Documentação técnica: `README.md` na pasta do projeto
