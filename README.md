# LIAC

Compilador para a linguagem LIA. Projeto da disciplina de Compiladores do Curso de Ciência da Computação.

## Execução (somente do compilador)

Após acessar a raiz do projeto, execute o comando:

```
cargo run --release -- /caminho/para/o/arquivo/de/entrada.lia
```

Ou, para executar em modo debug, execute o comando:

```
cargo run -- /caminho/para/o/arquivo/de/entrada.lia
```

## Execução (compilador e programa LIA)

Observação: requer `llvm` e `clang` instalados na máquina.

Após acessar a raiz do projeto, execute o comando:

```
./run.sh /caminho/para/o/arquivo/de/entrada.lia
```

Um binário será produzido. Para executá-lo, basta usar o comando:

```
./main
```

## Sobre os testes disponibilizados

Na raiz do projeto há quatro arquivos ```.lia```. ```teste1.lia``` e ```teste2.lia``` são exemplos que não terminam em erro, enquanto ```teste3.lia``` e ```teste4.lia``` possuem caracteres não previstos pela linguagem e geram erro.

## Documentação

A documentação da linguagem se encontra no diretório `docs/`.

## Binário executável

Está disponível no diretório `bin/`.
