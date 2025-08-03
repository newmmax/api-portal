# 📚 Documentação - FC Data API

## Índice de Documentos

### 📖 Documentação Principal
- [**API Documentation**](API_DOCUMENTATION.md) - Documentação completa de todos os endpoints
- [**Deploy Guide**](DEPLOY_GUIDE.md) - Guia completo de deploy para produção
- [**README Principal**](../README.md) - Visão geral do projeto

### 🧪 Desenvolvimento e Testes
- [**Bases de Teste**](BASES_TESTE.md) - Como usar as bases de desenvolvimento
- [**Testes API Pedidos**](TESTES_API_PEDIDOS.md) - Exemplos práticos de uso da API
- [**Regras de Negócio**](REGRAS_NEGOCIO_PEDIDOS.md) - Todas as regras de validação

### 🔧 Configuração
- [**.env.example**](../.env.example) - Exemplo de configuração de ambiente

## 🚀 Quick Start

### 1. Configuração Inicial
```bash
# Clone o projeto
git clone [url-do-repositorio]
cd fc-data-api

# Configure o ambiente
cp .env.example .env
# Edite o .env com suas configurações

# Compile
cargo build --release
```

### 2. Executar
```bash
# Desenvolvimento
cargo run

# Produção
./target/release/fc-data-api
```

### 3. Testar
```bash
# Health check
curl http://localhost:8089/services/api1/health

# Login
curl -X POST http://localhost:8089/services/api1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "sua-senha"}'
```

## 📊 Arquitetura

```
docs/
├── API_DOCUMENTATION.md    # Referência completa da API
├── DEPLOY_GUIDE.md         # Guia de deploy
├── BASES_TESTE.md          # Configuração de testes
├── TESTES_API_PEDIDOS.md   # Exemplos de uso
├── REGRAS_NEGOCIO_PEDIDOS.md # Regras de negócio
└── README.md               # Este arquivo
```

## 💡 Dicas

### Para Desenvolvedores
1. Use as bases de teste (`sys_pedidos_teste`, `sigaofc_teste`)
2. Sempre teste com `cargo clippy` antes de commitar
3. Mantenha a documentação atualizada

### Para DevOps
1. Veja o [Deploy Guide](DEPLOY_GUIDE.md) para configuração completa
2. Configure monitoramento do endpoint `/health`
3. Use HTTPS em produção sempre

### Para Usuários da API
1. Comece pela [API Documentation](API_DOCUMENTATION.md)
2. Teste com os exemplos em [Testes API](TESTES_API_PEDIDOS.md)
3. Entenda as [Regras de Negócio](REGRAS_NEGOCIO_PEDIDOS.md)

## 🆘 Suporte

- **Issues**: Abra uma issue no GitHub
- **Documentação**: Este diretório `/docs`
- **Logs**: Configure `RUST_LOG=debug` para mais detalhes

---

Última atualização: Janeiro 2025
