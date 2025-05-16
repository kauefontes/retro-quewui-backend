# Retro Quewui Backend

API backend em Rust para o portfolio estilo retro-tech "Quewui".

## Tecnologias

- Rust
- Actix Web
- Serde (serialização/deserialização JSON)
- CORS para integração com o frontend

## Estrutura do Projeto

```
src/
├── main.rs           # Ponto de entrada da aplicação
├── models/           # Modelos de dados
│   ├── experience.rs
│   ├── github_stats.rs
│   ├── mod.rs
│   ├── post.rs
│   ├── project.rs
│   └── skill.rs
└── routes/           # Rotas da API
    ├── experiences.rs
    ├── github_stats.rs
    ├── mod.rs
    ├── posts.rs
    ├── projects.rs
    └── skills.rs
```

## Rotas da API

- `GET /projects` - Lista todos os projetos
- `GET /projects/{id}` - Obtém um projeto específico
- `GET /experiences` - Lista todas as experiências profissionais
- `GET /experiences/{id}` - Obtém uma experiência específica
- `GET /skills` - Lista todas as habilidades
- `GET /posts` - Lista todos os posts do blog
- `GET /posts/{id}` - Obtém um post específico
- `GET /github-stats` - Obtém estatísticas do GitHub

## Requisitos

- Rust 1.75 ou superior
- Cargo (gerenciador de pacotes do Rust)

## Instalação e Execução

1. Clone o repositório:

   ```bash
   git clone https://github.com/seu-usuario/retro-quewui-backend.git
   cd retro-quewui-backend
   ```

2. Compile e execute em modo de desenvolvimento:

   ```bash
   cargo run
   ```

3. Ou compile em modo de produção:
   ```bash
   cargo build --release
   ./target/release/retro-quewui-backend
   ```

## Variáveis de Ambiente

Configure as seguintes variáveis no arquivo `.env` na raiz do projeto:

- `HOST` - Endereço IP do servidor (padrão: 127.0.0.1)
- `PORT` - Porta para a API (padrão: 8080)
- `FRONTEND_URL` - URL do frontend para configuração de CORS (padrão: http://localhost:5173)
- `RUST_LOG` - Nível de log (padrão: info)

## Desenvolvimento

Para desenvolvimento local, o servidor estará disponível em:

```
http://localhost:8080
```

O backend atualmente utiliza dados mockados para desenvolvimento. Em uma implementação futura, pode ser conectado a um banco de dados.
