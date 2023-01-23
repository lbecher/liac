# LIAC

Compilador para a linguagem LIA.

## Execução

Após acessar a raiz do projeto, execute o comando:

```
cargo run --release -- /caminho/para/o/arquivo/de/entrada.lia
```

Ou, para executar em modo debug, execute o comando:

```
cargo run -- /caminho/para/o/arquivo/de/entrada.lia
```

## Sobre os testes disponibilizados

Na raiz do projeto há quatro arquivos ```.lia```. ```teste1.lia``` e ```teste2.lia``` são exemplos que não terminam em erro, enquanto ```teste3.lia``` e ```teste4.lia``` possuem caracteres não previstos pela linguagem e geram erro.

## Gramática

```
S -> A
S -> A S
A -> AbreBlocoDeCodigo B FechaBlocoDeCodigo
B -> C
B -> C B
C -> AbreBlocoCondicional B FechaBlocoCondicional
C -> D DoisPontos E PontoEVirgula
D -> Bloc
D -> Set
D -> Print
D -> Scan
D -> TipoDeVariavel
E -> F
E -> F Virgula E
F -> G
F -> IdDeVariavel
F -> IdDeBloco
F -> Caractere
F -> Numero
F -> String
G -> Operador AbreParenteses H FechaParenteses
G -> Operador AbreParenteses H Virgula H FechaParenteses
H -> G
H -> IdDeVariavel
H -> Numero
```

## Gramática Aumentada

```
00) S' -> S
01) S -> A
02) S -> A S
03) A -> AbreBlocoDeCodigo B FechaBlocoDeCodigo
04) B -> C
05) B -> C B
06) C -> AbreBlocoCondicional B FechaBlocoCondicional
07) C -> D 
08) D -> Bloc DoisPontos IdDeBloco PontoEVirgula
09) D -> Set DoisPontos F PontoEVirgula
10) D -> Print
11) D -> Scan DoisPontos IdDeVariavel PontoEVirgula
12) D -> TipoDeVariavel DoisPontos E PontoEVirgula
13) E -> F
14) E -> F Virgula E
15) F -> G
16) F -> IdDeVariavel
18) F -> Caractere
19) F -> Numero
20) F -> String
21) G -> Operador AbreParenteses H FechaParenteses
22) G -> Operador AbreParenteses H Virgula H FechaParenteses
23) H -> G
24) H -> IdDeVariavel
25) H -> Numero
```


## BNF

```
<S'> ::= <S>
<S> ::= <A>
<S> ::= <A> <S>
<A> ::= AbreBlocoDeCodigo <B> FechaBlocoDeCodigo
<B> ::= <C>
<B> ::= <C> <B>
<C> ::= AbreBlocoCondicional <B> FechaBlocoCondicional
<C> ::= <D> DoisPontos <E> PontoEVirgula
<D> ::= Bloc
<D> ::= Set
<D> ::= Print
<D> ::= Scan
<D> ::= TipoDeVariavel
<E> ::= <F>
<E> ::= <F> Virgula <E>
<F> ::= <G>
<F> ::= IdDeVariavel
<F> ::= IdDeBloco
<F> ::= Caractere
<F> ::= Numero
<F> ::= String
<G> ::= Operador AbreParenteses <H> FechaParenteses
<G> ::= Operador AbreParenteses <H> Virgula <H> FechaParenteses
<H> ::= <G>
<H> ::= IdDeVariavel
<H> ::= Numero
```

## Autômato

| Estado | Itens | Transições |
|---|---|---|
| ```I0``` | ```S' -> .S```<br>```S -> .A```<br>```S -> .A S```<br>```A -> .AbreBlocoDeCodigo B FechaBlocoDeCodigo``` | ```δ(I0, S) = I1```<br>```δ(I0, A) = I2```<br>```δ(I0, AbreBlocoDeCodigo) = I3``` |
| ```I1``` | ```S' -> S.``` |   |
| ```I2``` | ```S -> A .```<br>```S -> A .S```<br>```S -> .A```<br>```S -> .A S```<br>```A -> .AbreBlocoDeCodigo B FechaBlocoDeCodigo``` | ```δ(I2, S) = I4```<br>```δ(I2, A) = I2```<br>```δ(I2, AbreBlocoDeCodigo) = I3``` |
| ```I3``` | ```A -> AbreBlocoDeCodigo .B FechaBlocoDeCodigo```<br>```B -> .C```<br>```B -> .C B```<br>```C -> .AbreBlocoCondicional B FechaBlocoCondicional```<br>```C -> .D DoisPontos E PontoEVirgula```<br>```D -> .Bloc```<br>```D -> .Set```<br>```D -> .Print```<br>```D -> .Scan```<br>```D -> .TipoDeVariavel``` | ```δ(I3, B) = I5```<br>```δ(I3, C) = I6```<br>```δ(I3, AbreBlocoCondicional) = I7```<br>```δ(I3, D) = I8```<br>```δ(I3, Bloc) = I9```<br>```δ(I3, Set) = I10```<br>```δ(I3, Print) = I11```<br>```δ(I3, Scan) = I12```<br>```δ(I3, TipoDeVariavel) = I13``` |
| ```I4``` | ```S -> A S.``` |   |
| ```I5``` | ```A -> AbreBlocoDeCodigo B .FechaBlocoDeCodigo``` | ```δ(I5, FechaBlocoDeCodigo) = I14``` |
| ```I6``` | ```B -> C.```<br>```B -> C .B```<br>```B -> .C```<br>```B -> .C B```<br>```C -> .AbreBlocoCondicional B FechaBlocoCondicional```<br>```C -> .D DoisPontos E PontoEVirgula```<br>```D -> .Bloc```<br>```D -> .Set```<br>```D -> .Print```<br>```D -> .Scan```<br>```D -> .TipoDeVariavel``` | ```δ(I6, B) = I15```<br>```δ(I6, C) = I6```<br>```δ(I6, AbreBlocoCondicional) = I7```<br>```δ(I6, D) = I8```<br>```δ(I6, Bloc) = I9```<br>```δ(I6, Set) = I10```<br>```δ(I6, Print) = I11```<br>```δ(I6, Scan) = I12```<br>```δ(I6, TipoDeVariavel) = I13``` |
| ```I7``` | ```C -> AbreBlocoCondicional .B FechaBlocoCondicional```<br>```B -> .C```<br>```B -> .C B```<br>```C -> .AbreBlocoCondicional B FechaBlocoCondicional```<br>```C -> .D DoisPontos E PontoEVirgula```<br>```D -> .Bloc```<br>```D -> .Set```<br>```D -> .Print```<br>```D -> .Scan```<br>```D -> .TipoDeVariavel``` | ```δ(I7, B) = I16```<br>```δ(I7, C) = I6```<br>```δ(I7, AbreBlocoCondicional) = I7```<br>```δ(I7, D) = I8```<br>```δ(I7, Bloc) = I9```<br>```δ(I7, Set) = I10```<br>```δ(I7, Print) = I11```<br>```δ(I7, Scan) = I12```<br>```δ(I7, TipoDeVariavel) = I13``` |
| ```I8``` | ```C -> D .DoisPontos E PontoEVirgula``` | ```δ(I8, DoisPontos) = I17``` |
| ```I9``` | ```D -> Bloc.``` |   |
| ```I10``` | ```D -> Set.``` |   |
| ```I11``` | ```D -> Print.``` |   |
| ```I12``` | ```D -> Scan.``` |   |
| ```I13``` | ```D -> TipoDeVariave.``` |   |
| ```I14``` | ```A -> AbreBlocoDeCodigo B FechaBlocoDeCodigo.``` |   |
| ```I15``` | ```B -> C B.``` |   |
| ```I16``` | ```C -> AbreBlocoCondicional B .FechaBlocoCondicional``` | ```δ(I16, FechaBlocoCondicional) = I18``` |
| ```I17``` | ```C -> D DoisPontos .E PontoEVirgula ```<br>```E -> .F ```<br>```E -> .F Virgula E ```<br>```F -> .G ```<br>```F -> .IdDeVariavel ```<br>```F -> .IdDeBloco ```<br>```F -> .Caractere ```<br>```F -> .Numero ```<br>```F -> .String ```<br>```G -> .Operador AbreParenteses H FechaParenteses ```<br>```G -> .Operador AbreParenteses H Virgula H FechaParenteses``` | ```δ(I17, E) = I19 ```<br>```δ(I17, F) = I20 ```<br>```δ(I17, G) = I21 ```<br>```δ(I17, IdDeVariavel) = I22 ```<br>```δ(I17, IdDeBloco) = I23 ```<br>```δ(I17, Caractere) = I24 ```<br>```δ(I17, Numero) = I25 ```<br>```δ(I17, String) = I26 ```<br>```δ(I17, Operador) = I27``` |
| ```I18``` | ```C -> AbreBlocoCondicional B FechaBlocoCondicional.``` |   |
| ```I19``` | ```C -> D DoisPontos E .PontoEVirgula``` | ```δ(I19, PontoEVirgula) = I28``` |
| ```I20``` | ```E -> F. ```<br>```E -> F .Virgula E``` | ```δ(I20, Virgula) = I29``` |
| ```I21``` | ```F -> G.``` |   |
| ```I22``` | ```F -> IdDeVariavel.``` |   |
| ```I23``` | ```F -> IdDeBloco.``` |   |
| ```I24``` | ```F -> Caractere.``` |   |
| ```I25``` | ```F -> Numero.``` |   |
| ```I26``` | ```F -> String.``` |   |
| ```I27``` | ```G -> Operador .AbreParenteses H FechaParenteses ```<br>```G -> Operador .AbreParenteses H Virgula H FechaParenteses``` | ```δ(I27, AbreParenteses) = I30``` |
| ```I28``` | ```C -> D DoisPontos E PontoEVirgula.``` |   |
| ```I29``` | ```E -> F Virgula .E ```<br>```E -> .F ```<br>```E -> .F Virgula E ```<br>```F -> .G ```<br>```F -> .IdDeVariavel ```<br>```F -> .IdDeBloco ```<br>```F -> .Caractere ```<br>```F -> .Numero ```<br>```F -> .String ```<br>```G -> .Operador AbreParenteses H FechaParenteses ```<br>```G -> .Operador AbreParenteses H Virgula H FechaParenteses``` | ```δ(I29, E) = I31 ```<br>```δ(I29, F) = I20 ```<br>```δ(I29, G) = I21 ```<br>```δ(I29, IdDeVariavel) = I22 ```<br>```δ(I29, IdDeBloco) = I23 ```<br>```δ(I29, Caractere) = I24 ```<br>```δ(I29, Numero) = I25 ```<br>```δ(I29, String) = I26 ```<br>```δ(I29, Operador) = I27``` |
| ```I30``` | ```G -> Operador AbreParenteses .H FechaParenteses ```<br>```G -> Operador AbreParenteses .H Virgula H FechaParenteses ```<br>```H -> .G ```<br>```H -> .IdDeVariavel ```<br>```H -> .Numero ```<br>```G -> .Operador AbreParenteses H FechaParenteses ```<br>```G -> .Operador AbreParenteses H Virgula H FechaParenteses``` | ```δ(I30, H) = I32 ```<br>```δ(I30, G) = I21 ```<br>```δ(I30, IdDeVariavel) = I33 ```<br>```δ(I30, Numero) = I34 ```<br>```δ(I30, Operador) = I27``` |
| ```I31``` | ```E -> F Virgula E.``` |   |
| ```I32``` | ```G -> Operador AbreParenteses H .FechaParenteses ```<br>```G -> Operador AbreParenteses H .Virgula H FechaParenteses``` | ```δ(I32, FechaParenteses) = I35 ```<br>```δ(I32, Virgula) = I36``` |
| ```I33``` | ```H -> IdDeVariavel.``` |   |
| ```I34``` | ```H -> Numero.``` |   |
| ```I35``` | ```G -> Operador AbreParenteses H FechaParenteses.``` |   |
| ```I36``` | ```G -> Operador AbreParenteses H Virgula .H FechaParenteses ```<br>```H -> .G ```<br>```H -> .IdDeVariavel ```<br>```H -> .Numero ```<br>```G -> .Operador AbreParenteses H FechaParenteses ```<br>```G -> .Operador AbreParenteses H Virgula H FechaParenteses``` | ```δ(I36, H) = I37 ```<br>```δ(I36, G) = I21 ```<br>```δ(I36, IdDeVariavel) = I33 ```<br>```δ(I36, Numero) = I34 ```<br>```δ(I36, Operador) = I27``` |
| ```I37``` | ```G -> Operador AbreParenteses H Virgula H .FechaParenteses``` | ```δ(I37, FechaParenteses) = I38``` |
| ```I38``` | ```G -> Operador AbreParenteses H Virgula H FechaParenteses.``` |   |

## Tabela RL

|  Estado | Virgula | DoisPontos | PontoEVirgula | AbreParenteses | FechaParenteses | Set | Print | Scan | Bloc | Operador | TipoDeVariavel | IdDeVariavel | IdDeBloco | AbreBlocoCondicional | FechaBlocoCondicional | AbreBlocoDeCodigo | FechaBlocoDeCodigo | Caractere | Numero | String | $ | S | A | B | C | D | E | F | G | H |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| I0 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I1 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I2 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I3 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I4 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I5 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I6 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I7 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I8 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I9 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I10 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I11 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I12 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I13 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I14 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I15 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I16 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I17 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I18 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I19 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I20 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I21 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I22 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I23 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I24 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I25 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I26 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I27 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I28 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I29 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I30 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I31 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I32 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I33 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I34 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I35 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I36 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I37 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| I38 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |