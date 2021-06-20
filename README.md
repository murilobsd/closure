# Rust closure

Exemplo simples de como usar **closure** em Rust e seus benefícios, o código
atual utiliza o pattern [Memoization](https://en.wikipedia.org/wiki/Memoization)
o qual trás vantagens quanto utilizamos funções que demandam muito
processamento e as mesmas são chamadas diversas vezes. 

Basicamente a closure é usada somente uma vez mantendo o __cache__ do primeiro
cálculo, caso um valor diferente seja passado para closure ele é usado como
chave do HashMap e o valor é a resultante do cálculo.

## Rodar

```
# 10 é a intensidade do exercício
cargo run -- 10
```

## Test

```
cargo test
```
