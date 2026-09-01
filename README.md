<div align="center">
  <img src="https://capsule-render.vercel.app/api?type=waving&color=0:ff4d4d,100:990000&height=250&section=header&text=MetaFR&fontSize=80&animation=fadeIn&fontAlignY=38&desc=Framework%20frontend%20em%20Rust&descAlignY=51&descAlign=50" />
  
  <br>
</div>

## 🇺🇸 English

### 🦀 Frontend Framework for Rust!

MetaFR is a framework that uses Tokio and Axum to generate HTML tags, making it possible to build websites using only Rust. The main HTML tags are converted into standard components.

The website is built with an AST using the builder pattern, and then it is optimized for delivery. With MetaFR, it's possible to create new components and parameters, allowing integration with HTMX, Bootstrap, among others.

### ✨ Features
- **100% Rust:** Build your frontend without writing JS/HTML.
- **AST & Builder Pattern:** Clean and predictable component structure.
- **Component-based:** Easy to create and reuse custom components.
- **Ecosystem integration:** Works well with HTMX, Bootstrap, and more.

### 🚀 Installation & Usage

Add MetaFR to your `Cargo.toml`:
```toml
[dependencies]
metafr = "*" # Check for the latest version
```

### ⚡ Speed

Being a Rust framework, it focuses on application speed. When the software starts, the AST is built and generates HTML files that are stored in cache, ready to be sent. This build process is fast because it doesn't have the overhead of a garbage collector. The distribution of the site, in turn, happens almost completely statically in a lightweight and highly performant environment.

---

## 🇧🇷 Português

### 🦀 Framework Frontend para Rust!

O MetaFR é um framework que usa o Tokio e o Axum para fazer a geração de tags HTML e possibilitar a criação de sites apenas com Rust. As principais tags HTML são convertidas em componentes padrão.

O site é construído com uma AST usando *builder pattern* e, em seguida, é otimizado para a entrega. Com o MetaFR, é possível criar novos componentes e novos parâmetros, permitindo a integração com HTMX, Bootstrap, entre outros.

### ✨ Funcionalidades
- **100% Rust:** Construa seu frontend sem escrever JS/HTML.
- **AST & Builder Pattern:** Estrutura de componentes limpa e previsível.
- **Baseado em Componentes:** Facilidade para criar e reutilizar componentes.
- **Integração fácil:** Funciona perfeitamente com HTMX, Bootstrap e outros.

### 🚀 Instalação e Uso

Adicione o MetaFR no seu `Cargo.toml`:
```toml
[dependencies]
metafr = "*" # Verifique a versão mais recente
```

### ⚡ Velocidade

Por ser um framework Rust, ele foca na velocidade da aplicação. Quando o software é iniciado, a AST é construída e gera arquivos HTML que ficam armazenados, prontos para serem enviados. Essa construção é rápida, pois a linguagem não possui o peso de um *garbage collector*. A distribuição do site, por sua vez, acontece quase que de forma completamente estática, em um ambiente mais leve e performático.

---

### 💻 Exemplo / Example (Hello World)

```rust
use metafr::{Page, document::{Document, create_document}, get, start };
use metafr::components::{
    typography,
    scope
};
use metafr::Html;

fn main() {

    let paginas = vec![
        Page {
            path: "/".to_string(),
            method: get(|| async { 
                // Home page!
                let home: Document = create_document(
                    scope::scope_create()
                    .set_children(
                    vec![
                        typography::typography_create()
                        .set_text("Hello World!")
                        .build(),

                        typography::typography_create()
                        .set_text("Hello Github")
                        .build()
                    ]
                )
                    .build()
                );

                Html(home.render())
            }),
        }
    ];

    start(&paginas);

}
```

Para testar o exemplo acima, basta rodar / To test the example above, just run:
```bash
cargo run
```

---

### 🔓 Licença / License

Este projeto é de código aberto e livre para qualquer pessoa usar, estudar e modificar (**Licença MIT**). Sem restrições de uso!

This project is completely open and free for anyone to use, study, and modify (**MIT License**). No usage restrictions!

---

## ⚠️ Aviso / Warning (WIP)

**🇧🇷 PT:** Por favor, **não utilize este framework em cenários reais de produção ainda**. O projeto está em fase inicial de testes e prototipação. No momento, **não estou aceitando contribuições** (Pull Requests), pois ainda estou desenvolvendo a primeira versão principal sozinho. Apenas acompanhe o desenvolvimento!

**🇺🇸 EN:** Please **do not use this framework in a real or production scenario yet**. The project is currently just in its early testing/prototype phase. At this moment, **it is not open for contributions**, as I am still writing the first core version by myself. Just stay tuned!
