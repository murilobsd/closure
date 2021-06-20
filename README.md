# Rust clousure

Exemplo simples de como usar **closure** em Rust e seus benefícios, o código atual
exibe o pattern [Memoization](https://en.wikipedia.org/wiki/Memoization) o qual tras
vantagens em regras de negócios que demandam muito processamento.

Basicamente a clousure é usada somente uma vez mantendo o __cache__ do primeiro calculo,
caso um novo valor seja calculado é inserido em um HashMap o novo valor para aquela calculo.

até o momento não estamos usando Generics, mas será adicionado futuramente =]

## Rodar

```
cargo run
```

## Test

```
cargo test
```
