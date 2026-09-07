# validador_crate_rust

Crate em Rust para validação de CPF, com verificação completa dos dígitos verificadores e tratamento de sequências repetidas.

## Instalação

Adicione ao `Cargo.toml` do seu projeto:

```toml
[dependencies]
validador_crate_rust = "0.1.0"
```

Ou, via linha de comando:

```bash
cargo add validador_crate_rust
```

## Como utilizar

```rust
use validador_crate_rust as vd;
use std::io;

fn main() {
    println!("Digite um CPF");

    let mut cpf = String::new();

    match io::stdin().read_line(&mut cpf) {
        Ok(_) => {
            println!("O CPF digitado é: {}", cpf.trim());
        },
        Err(error) => {
            println!("Erro ao ler a entrada: {}", error);
            return;
        }
    }

    let validado: bool = vd::validar_cpf(cpf.as_str());

    if validado {
        println!("O CPF é válido");
    } else {
        println!("O CPF é inválido");
    }
}
```

A função aceita o CPF com ou sem formatação:

```rust
vd::validar_cpf("529.982.247-25"); // true
vd::validar_cpf("52998224725");    // true
vd::validar_cpf("111.111.111-11"); // false (sequência repetida)
```

## Como funciona

1. Extrai apenas os dígitos da string recebida, ignorando pontos e traço.
2. Verifica se o resultado tem exatamente 11 dígitos.
3. Rejeita sequências repetidas (ex: `111.111.111-11`), que passariam no cálculo matemático mas não são CPFs válidos.
4. Calcula os dois dígitos verificadores com base nos 9 primeiros números, usando pesos decrescentes e o resto da divisão por 11.
5. Compara os dígitos calculados com os informados no CPF.

## Testes

O crate já inclui testes unitários. Para rodá-los:

```bash
cargo test
```

## Licença

MIT
