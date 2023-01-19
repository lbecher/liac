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
S' -> S
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

# Autômato

| Estado | Itens | Transições |
|---|---|---|
| ```I0``` | ```S' -> .S```<br>```S -> .A```<br>```S -> .A S```<br>```A -> .AbreBlocoDeCodigo B FechaBlocoDeCodigo``` | ```δ(i0, S) = i1```<br>```δ(i0, A) = i2```<br>```δ(i0, AbreBlocoDeCodigo) = i3``` |
| ```I1``` | ```S' -> S.``` |   |
| ```I2``` | ```S -> A .```<br>```S -> A .S```<br>```S -> .A```<br>```S -> .A S```<br>```A -> .AbreBlocoDeCodigo B FechaBlocoDeCodigo``` | ```δ(i2, S) = i4```<br>```δ(i2, A) = i2```<br>```δ(i2, AbreBlocoDeCodigo) = i3``` |
| ```I3``` | ```A -> AbreBlocoDeCodigo .B FechaBlocoDeCodigo```<br>```B -> .C```<br>```B -> .C B```<br>```C -> .AbreBlocoCondicional B FechaBlocoCondicional```<br>```C -> .D DoisPontos E PontoEVirgula```<br>```D -> .Bloc```<br>```D -> .Set```<br>```D -> .Print```<br>```D -> .Scan```<br>```D -> .TipoDeVariavel``` | ```δ(i3, B) = i5```<br>```δ(i3, C) = i6```<br>```δ(i3, AbreBlocoCondicional) = i7```<br>```δ(i3, D) = i8```<br>```δ(i3, Bloc) = i9```<br>```δ(i3, Set) = i10```<br>```δ(i3, Print) = i11```<br>```δ(i3, Scan) = i12```<br>```δ(i3, TipoDeVariavel) = i13``` |
| ```I4``` | ```S -> A S.``` |   |
| ```I5``` | ```A -> AbreBlocoDeCodigo B .FechaBlocoDeCodigo``` | ```δ(i5, FechaBlocoDeCodigo) = i14``` |
| ```I6``` | ```B -> C.```<br>```B -> C .B```<br>```B -> .C```<br>```B -> .C B```<br>```C -> .AbreBlocoCondicional B FechaBlocoCondicional```<br>```C -> .D DoisPontos E PontoEVirgula```<br>```D -> .Bloc```<br>```D -> .Set```<br>```D -> .Print```<br>```D -> .Scan```<br>```D -> .TipoDeVariavel``` | ```δ(i6, B) = i15```<br>```δ(i6, C) = i6```<br>```δ(i6, AbreBlocoCondicional) = i7```<br>```δ(i6, D) = i8```<br>```δ(i6, Bloc) = i9```<br>```δ(i6, Set) = i10```<br>```δ(i6, Print) = i11```<br>```δ(i6, Scan) = i12```<br>```δ(i6, TipoDeVariavel) = i13``` |
| ```I7``` | ```C -> AbreBlocoCondicional .B FechaBlocoCondicional```<br>```B -> .C```<br>```B -> .C B```<br>```C -> .AbreBlocoCondicional B FechaBlocoCondicional```<br>```C -> .D DoisPontos E PontoEVirgula```<br>```D -> .Bloc```<br>```D -> .Set```<br>```D -> .Print```<br>```D -> .Scan```<br>```D -> .TipoDeVariavel``` | ```δ(i7, B) = i16```<br>```δ(i7, C) = i6```<br>```δ(i7, AbreBlocoCondicional) = i7```<br>```δ(i7, D) = i8```<br>```δ(i7, Bloc) = i9```<br>```δ(i7, Set) = i10```<br>```δ(i7, Print) = i11```<br>```δ(i7, Scan) = i12```<br>```δ(i7, TipoDeVariavel) = i13``` |
| ```I8``` | ```C -> D .DoisPontos E PontoEVirgula``` | ```δ(i8, DoisPontos) = i17``` |
| ```I9``` | ```D -> Bloc.``` |   |
| ```I10``` | ```D -> Set.``` |   |
| ```I11``` | ```D -> Print.``` |   |
| ```I12``` | ```D -> Scan.``` |   |
| ```I13``` | ```D -> TipoDeVariave.``` |   |
| ```I14``` | ```A -> AbreBlocoDeCodigo B FechaBlocoDeCodigo.``` |   |
| ```I15``` | ```B -> C B.``` |   |
| ```I16``` | ```C -> AbreBlocoCondicional B .FechaBlocoCondicional``` | ```δ(i16, FechaBlocoCondicional) = i18``` |
| ```I17``` | ```C -> D DoisPontos .E PontoEVirgula ```<br>```E -> .F ```<br>```E -> .F Virgula E ```<br>```F -> .G ```<br>```F -> .IdDeVariavel ```<br>```F -> .IdDeBloco ```<br>```F -> .Caractere ```<br>```F -> .Numero ```<br>```F -> .String ```<br>```G -> .Operador AbreParenteses H FechaParenteses ```<br>```G -> .Operador AbreParenteses H Virgula H FechaParenteses``` | ```δ(i17, E) = i19 ```<br>```δ(i17, F) = i20 ```<br>```δ(i17, G) = i21 ```<br>```δ(i17, IdDeVariavel) = i22 ```<br>```δ(i17, IdDeBloco) = i23 ```<br>```δ(i17, Caractere) = i24 ```<br>```δ(i17, Numero) = i25 ```<br>```δ(i17, String) = i26 ```<br>```δ(i17, Operador) = i27``` |
| ```I18``` | ```C -> AbreBlocoCondicional B FechaBlocoCondicional.``` |   |
| ```I19``` | ```C -> D DoisPontos E .PontoEVirgula``` | ```δ(i19, PontoEVirgula) = i28``` |
| ```I20``` | ```E -> F. ```<br>```E -> F .Virgula E``` | ```δ(i20, Virgula) = i29``` |
| ```I21``` | ```F -> G.``` |   |
| ```I22``` | ```F -> IdDeVariavel.``` |   |
| ```I23``` | ```F -> IdDeBloco.``` |   |
| ```I24``` | ```F -> Caractere.``` |   |
| ```I25``` | ```F -> Numero.``` |   |
| ```I26``` | ```F -> String.``` |   |
| ```I27``` | ```G -> Operador .AbreParenteses H FechaParenteses ```<br>```G -> Operador .AbreParenteses H Virgula H FechaParenteses``` | ```δ(i27, AbreParenteses) = i30``` |
| ```I28``` | ```C -> D DoisPontos E PontoEVirgula.``` |   |
| ```I29``` | ```E -> F Virgula .E ```<br>```E -> .F ```<br>```E -> .F Virgula E ```<br>```F -> .G ```<br>```F -> .IdDeVariavel ```<br>```F -> .IdDeBloco ```<br>```F -> .Caractere ```<br>```F -> .Numero ```<br>```F -> .String ```<br>```G -> .Operador AbreParenteses H FechaParenteses ```<br>```G -> .Operador AbreParenteses H Virgula H FechaParenteses``` | ```δ(i29, E) = i31 ```<br>```δ(i29, F) = i20 ```<br>```δ(i29, G) = i21 ```<br>```δ(i29, IdDeVariavel) = i22 ```<br>```δ(i29, IdDeBloco) = i23 ```<br>```δ(i29, Caractere) = i24 ```<br>```δ(i29, Numero) = i25 ```<br>```δ(i29, String) = i26 ```<br>```δ(i29, Operador) = i27``` |
| ```I30``` | ```G -> Operador AbreParenteses .H FechaParenteses ```<br>```G -> Operador AbreParenteses .H Virgula H FechaParenteses ```<br>```H -> .G ```<br>```H -> .IdDeVariavel ```<br>```H -> .Numero ```<br>```G -> .Operador AbreParenteses H FechaParenteses ```<br>```G -> .Operador AbreParenteses H Virgula H FechaParenteses``` | ```δ(i30, H) = i32 ```<br>```δ(i30, G) = i21 ```<br>```δ(i30, IdDeVariavel) = i33 ```<br>```δ(i30, Numero) = i34 ```<br>```δ(i30, Operador) = i27``` |
| ```I31``` | ```E -> F Virgula E.``` |   |
| ```I32``` | ```G -> Operador AbreParenteses H .FechaParenteses ```<br>```G -> Operador AbreParenteses H .Virgula H FechaParenteses``` | ```δ(i32, FechaParenteses) = i35 ```<br>```δ(i32, Virgula) = i36``` |
| ```I33``` | ```H -> IdDeVariavel.``` |   |
| ```I34``` | ```H -> Numero.``` |   |
| ```I35``` | ```G -> Operador AbreParenteses H FechaParenteses.``` |   |
| ```I36``` | ```G -> Operador AbreParenteses H Virgula .H FechaParenteses ```<br>```H -> .G ```<br>```H -> .IdDeVariavel ```<br>```H -> .Numero ```<br>```G -> .Operador AbreParenteses H FechaParenteses ```<br>```G -> .Operador AbreParenteses H Virgula H FechaParenteses``` | ```δ(i36, H) = i37 ```<br>```δ(i36, G) = i21 ```<br>```δ(i36, IdDeVariavel) = i33 ```<br>```δ(i36, Numero) = i34 ```<br>```δ(i36, Operador) = i27``` |
| ```I37``` | ```G -> Operador AbreParenteses H Virgula H .FechaParenteses``` | ```δ(i37, FechaParenteses) = i38``` |
| ```I38``` | ```G -> Operador AbreParenteses H Virgula H FechaParenteses.``` |   |