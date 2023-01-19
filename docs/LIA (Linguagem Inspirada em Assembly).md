# LIA (Linguagem Inspirada em Assembly)

A linguagem de programação LIA é inspirada na linguagem Assembly e sua estrutura de codificação é dividida em blocos. Nela há dois blocos obrigatórios, sendo o primeiro bloco dedicado à declaração de variáveis, chamado DATA, e o segundo dedicado ao código principal do programa, chamado MAIN, onde a execução tem início. Os demais blocos são blocos reutilizáveis que funcionam de forma semelhante a macros em outras linguagens.

Criadores: Luiz Fernando Becher de Araujo e Marcos Augusto Campagnaro Mucelini.

## Características

LIA é uma linguagem com características herdadas tanto da linguagem Assembly quanto de linguagens de programação de alto nível. Sendo algumas delas:

- Codificação simples;
- Blocos reutilizáveis de código;
- Suporte apenas a variáveis globais;
- Operadores com número mínimo de parâmetros.

### Codificação simples

LIA tem objetivo ser uma linguagem de mais alto nível que a linguagem Assembly e, por isso, implementa alguns recursos de sintaxe para facilitar o uso de operadores. Desta forma, pode ser utilizada como uma linguagem introdutória a linguagens de baixo nível, justamente por causa das características provenientes da Assembly, porém sem se preocupar com alguns detalhes como o gerenciamento de registradores, saltos e chamadas de sistema.

### Blocos reutilizáveis de código

A linguagem Assembly trata funções utilizando-se de saltos e trocas de contexto. Entretanto, apenas os registradores são afetados por esse processo. As variáveis declaradas no programa permanecem no contexto global, o que não acontece em linguagens de alto nível que permitem a declaração de variáveis locais, onde o contexto é importante. Nesta primeira versão da LIA, seguindo o exemplo do Assembly, não há suporte para declaração de variáveis locais, pois demandaria a implementação de algum mecanismo para criar contextos entre as variáveis declaradas localmente.

Porém, possuir algum tipo de estrutura para reaproveitamento de código é importante quando se tenta evitar a escrita repetitiva de procedimentos. Pensando nisso, a LIA implementa blocos reutilizáveis de código que funcionam de forma semelhante a macros em linguagens de alto nível e podem ser comparados ao uso de saltos na linguagem Assembly.

Se houver a necessidade de reaproveitamento de código, basta criar um ou mais blocos reutilizáveis, seguindo o mesmo padrão de declaração visto no bloco de dados e no bloco principal, alterando somente o nome para algo diferente de DATA ou MAIN. A seguir temos um exemplo de declaração de um bloco reutilizável de código. Mais a frente falaremos sobre os comandos contidos neste exemplo.

```
#DATA:
    INT32 x, y, resultado;
#DATA;

#MAIN:
    BLOC: SOMA;
#MAIN;

#SOMA:
    SET: resultado, 0;
    SET: resultado, ADD(x, y);
#SOMA;
```

### Suporte apenas a variáveis globais

Como dito na seção anterior, LIA não oferece suporte à declaração de variáveis locais (dentro de blocos que não seja o bloco de dados), pois não há suporte a contextos de variáveis locais nesta primeira versão. Sendo assim, o programador deve ficar atento às variáveis necessárias para o seu programa, tomando cuidado para não sobrescrever o valor de uma variável importante durante a chamada de algum bloco reutilizável.

Por isso, é recomendado criar variáveis no bloco de dados que serão operadas somente no interior dos blocos reutilizáveis. O exemplo abaixo demonstra isso na prática.

```
#DATA:
    UINT32: var1, var2, resultado;
    UINT32: soma_var1, soma_var2, soma_resultado;
#DATA;

#MAIN:
    !--
    qualquer modificação em var1 e var2
    --!

    SET: soma_var1, var1;
    SET: soma_var2, var2
    BLOC: SOMA;
    SET: resultado, soma_resultado;

    !--
    qualquer modificação em resultado
    --!
#MAIN:

#SOMA:
    SET: soma_resultado, ADD(soma_var1, soma_var2);
#SOMA;
```

### Operadores com número mínimo de parâmetros

Assim como acontece na linguagem Assembly, os operadores recebem apenas o número mínimo de parâmetros para cada operação. Isto é utilizado como um artifício para evitar a preocupação com precedência de operadores em nível de compilação.

Entretanto, LIA permite a realização de múltiplas operações na mesma linha de código, diferentemente da linguagem Assembly. Para que isto seja possível, os operadores são escritos como chamadas de função de linguagens de alto nível. Como os parâmetros dos operadores são envolvidos por parênteses, isto faz com que a precedência de operadores seja explicitada pelo programador.

Abaixo há um exemplo de operações lógicas e aritméticas encadeadas em uma linha. Mais detalhes sobre os operadores estarão descritos nas seções dedicadas a cada um.

```
#DATA:
    INT32: x, y, resultado;
#DATA

#MAIN:
    // resultado = resultado + 2;
    SET: resultado, ADD(resultado, 2);

    // resultado = resultado + (x + (resultado * 2));
    SET: resultado, ADD(resultado, ADD(x, MUL(resultado, 2)));
#MAIN;

#OPLOGICA:
    SET: x, 1;
    SET: y, 0;

    // if (x > y && resultado < y);
    #INZ: AND(A(x, y), B(resultado, y)):

        // conteúdo do if

    #INZ;

    // x > y && (resultado < y || resultado < x);
    AND(A(x, y), OR(B(resultado, y), B(resultado, x)));
#OPLOGICA;
```

### Demais características

LIA faz diferença entre letras maiúsculas e minúsculas. Todas as palavras reservadas são escritas em letras maiúsculas, assim como os nomes dos blocos reutilizáveis. Nos nomes dos blocos reutilizáveis é permitido o uso de underline e números, desde que não estejam no início. O nome das variáveis podem ser escritos somente com letras minúsculas, números e underlines, sendo que o primeiro caractere deve obrigatoriamente ser uma letra minúscula.

```
#DATA:
    INT64: a, b, c, resultado_soma;
#DATA;

#MAIN:
    SET: a, 64;
    SET: b, 32;
    SET: c, 16;
    BLOC: MINHA_SOMA;
#MAIN;

#MINHA_SOMA:
    SET: resultado_soma, ADD(a, ADD(b, c));
#MINHA_SOMA;
```

## Blocos

LIA possui blocos de código como a maioria das linguagens de programação. Aqui, eles são classificados nas seguintes categorias:

- Procedimentos;
- Estruturas de repetição/decisão;
- Bloco de dados;
- Código principal.

A seguir é apresentado um exemplo contendo o bloco de dados, o bloco principal e um procedimento na forma de bloco reutilizável. O bloco de dados (DATA) é utilizado para declarar as variáveis do programa e o bloco principal (MAIN) indica o bloco principal do programa, onde está o código que será executado primeiro. Logo, para cada programa, teremos um bloco de cada. É importante ressaltar que DATA e MAIN são consideradas palavras reservadas da linguagem.

```
#DATA:
    // Declarações de variaveis do programa
#DATA;

#MAIN:
    // Código principal aqui
#MAIN;

#MEUPROCEDIMENTO:
    // Um trecho de código aqui
#MEUPROCEDIMENTO;
```

### Início de bloco de código

No exemplo acima, vemos que o bloco reutilizável se chama `MEUPROCEDIMENTO`. O bloco começa com `#`, seguido do nome do bloco e, por fim, o caractere `:`. Com isto temos a delimitação do início de um bloco.

### Fim de bloco de código

Para delimitarmos o fim de um bloco. Utilizamos quase a mesma estrutura, porém com a diferença de que, ao invés de utilizarmos `:`, utilizamos `;`.

Essa estrutura de declaração e encerramento de blocos se repete por todos os tipos de blocos da linguagem.

### Blocos com parâmetros

Existe uma pequena diferença para estruturas de decisão e repetição, já que as mesmas podem recebee um conjunto de parâmetros.

A seguir temos um exemplo de um bloco utilizado em uma estrutura de repetição.

```
#WNZ: a:
    // Algum código aqui
#WZN;
```

Como podemos visualizar, a estrutura de repetição `#WNZ` (enquanto não zero) é um bloco que necessita de um parâmetro para funcionar corretamente e este parâmetros é passado após o primeiro `:`, para, então, vir o `:` final da declaração. Caso em alguma outra estrutura haja a necessidade de mais de um parâmetro, estes serão separados por vírgula. O encerramento do bloco permanece inalterado. A mesma estrutura se repete para a estrutura de decisão `#INZ`.

## Instrução BLOC

A instrução `BLOC` é utilizada para chamar um bloco reutilizável. Porém, não é possivel utilizá-lo para chamar blocos principais (DATA e MAIN).

```
BLOC: SOMAS_CONSECUTIVAS;
BLOC: FIBONACCI;
```

## Tipos de Dados

Os tipos de dados suportados pela linguagem são:

- INT8 - Utilizado para armazenar um inteiro de 8 bits sinalizado;
- INT16 - Utilizado para armazenar um inteiro de 16 bits sinalizado;
- INT32 - Utilizado para armazenar um inteiro de 32 bits sinalizado;
- INT64 - Utilizado para armazenar um inteiro de 64 bits sinalizado;
- UINT8 - Utilizado para armazenar um inteiro de 8 bits não sinalizado;
- UINT16 - Utilizado para armazenar um inteiro de 16 bits não sinalizado;
- UINT32 - Utilizado para armazenar um inteiro de 32 bits não sinalizado;
- UINT64 - Utilizado para armazenar um inteiro de 64 bits não sinalizado;
- CHR - Utilizado para armazenar um único caracter (equivalente ao UINT8);
- STR - Utilizado para armazenar uma cadeia de caracteres;

### Representação de valores lógicos (verdadeiro e falso)

Para simplificar, a LIA interpreta valores lógicos a partir de variśveis e valores inteiros (independentemente de serem sinalizados ou não sinalizados), onde o valor zero é interpretado como falso e qualquer valor diferente de zero é interpretado como verdadeiro.

## Operadores Relacionais

São funções relacionais que recebem dois valores como parâmetros. O primeiro obrigatoriamente é uma variável e o segundo pode ser tanto uma variável quanto um valor imediato. No cenário de ambos os valores serem variáveis, as variáveis devem ser do mesmo tipo.

Quando a comparação relacional é verdadeira, retorna-se verdadeiro, ou falso caso contrário.

### D (diferente)

Caso os parâmetros recebidos sejam cadeias de caracteres ou caracteres, a comparação será executada bitwise.

```
D(valor1, valor2)
```

### E (igual)

Caso os parâmetros recebidos sejam cadeias de caracteres ou caracteres, a comparação será executada bitwise.

```
E(valor1, valor2)
```

### A (maior)

```
A(valor1, valor2)
```

### AE (maior ou igual)

```
AE(valor1, valor2)
```

### B (menor)

```
B(valor1, valor2)
```

### BE (menor ou igual)

```
BE(valor1, valor2)
```

### SET (atribuição)

O operador `SET` é um operador de atribuição, portanto, seu papel é atribuir algum valor à alguma variável. Seu primeiro argumento é a variável onde a atribuição será realizada. O segundo pode ser um valor imediato a ser atribuído ou uma variável do mesmo tipo do primeiro argumento, que terá seu conteúdo copiado.

```
SET: var_de_destino, 128;
SET: var_de_destino, var_de_origem;
```

## Operações Aritméticas

São função de operações aritméticas que recebem dois valores como parâmetros. O primeiro obrigatoriamente é uma variável e o segundo pode ser tanto uma variável quanto um valor imediato.

No caso de ambos serem variáveis, devem ser do mesmo tipo.

### ADD (soma)

```
ADD(parcela1, parcela2)
```

### SUB (subtração)

```
SUB(minuendo, subtraendo)
```

### MUL (multiplicação)

```
MUL(fator1, fator2)
```

### DIV (divisão)

```
DIV(dividendo, divisor)
```

### DIVR (resto da divisão)

```
DIVR(dividendo, divisor)
```

## Operadores Lógicos

São um conjunto de funções de operações lógicas bitwise, ou seja, são aplicadas bit-a-bit. Estes operadores aceitam como parâmetros variáveis, valores imediatos ou operadores relacionais (exceto o de atribuição, `SET`).

### NOT (complementar)

A operação complementar recebe uma única variável, valor ou operação relacional com exceção da atribuição como parâmetro e retorna seu complementar.

```
NOT(valor)
```

### AND (e lógico) e OR (ou lógico)

Recebem dois valores como parâmetros. O primeiro obrigatoriamente é uma variável e o segundo pode ser tanto uma variável quanto um valor imediato.

No caso de ambos serem variáveis, devem ser do mesmo tipo.

```
AND(valor1, valor2)
OR(valor1, valor2)
```

## Delimitadores

### Delimitadores de comentários de linha

O início é representado pelos caracteres `//` e se estende até uma quebra de linha (`\n`).

```
#MAIN:
    // Código principal aqui
#MAIN;
```

### Delimitadores de bloco de comentário

O início do bloco é representado pelos caracteres `!--`, enquanto o fechamento é representado por `--!`.

```
!--
Texto de comentário que
ocupa mais de uma linha
--!

#MAIN:
    // Código principal aqui
#MAIN;
```

### Delimitador de fim de sentença

É representado pelo caractere `;`, que sinaliza o fim de uma sentença.

```
SET: x, 2;
SET: y, 4;
```

### Delimitadores de argumentos de função

É representado pelos caracteres `(`, para abertura, e `)`, para fechamento.

```
SET: x, AND(y, z);
SET: c, ADD(a, b);
```

### Delimitador de separação de argumentos

É representado pelo caractere `,`, que separa os argumentos de sentenças e de funções.

```
SET: x, 2;
SET: c, ADD(a, b);
```

### Delimitadores de blocos

Os blocos são delimitados por uma dupla de caracteres `#`, mais o nome do bloco. O delimitador de abertura termina com `:` e o delimitador de fechamento termina com `;`.

```
#INZ: x:
    ...
#INZ;
```

### Delimitadores de caracteres

Caracteres individuais são tratados entre `'`.

```
SET: caractere, 't';
```

### Delimitadores de cadeias de caracteres

Cadeias de caracteres (strings) são tratadas entre `"`. Quando esses delimitadores são utilizados, o caractere de fim de linha (\0) é adicionado ao fim da cadeia de caracteres automaticamente. Por isso, é importante declarar strings de tamanho `n + 1`.

```
SET: minha_string, "algum texto";
```

## Condicionais

Estruturas condicionais são utilizadas para executarmos trechos de código, somente se a condição controladora for satisfeita. Caso ela não seja cumprida, a execução continua normalmente após o encerramento do bloco condicional. As estruturas condicionais que a LIA suporta são:

### INZ (se não zero)

Essa condicional pode receber como parâmetro uma variável, valor imediato ou um operador lógico aritmético. E será ativada quando o valor da variável, no momento que a execução atingir essa condicional, for diferente de zero (representação de verdadeiro na LIA). Quando o valor da variável for zero (representação de falso), a execução pulará para a próxima linha após o fechamento do bloco INZ.

```
#INZ: x:
    ...
#INZ;
```

## Estruturas de Repetições

Estruturas de repetições delimitam trechos de códigos que serão repetidos enquanto a condição controladora for cumprida, uma vez que esta condição deixe de ser cumprida, o código continua a execução partindo da linha abaixo do encerramento do bloco de repetição.

### WNZ (enquanto não zero)

A operação WNZ recebe uma variável como parâmetro. E possui como objetivo repetir um trecho de código que se encontra dentro do bloco. O código é repetido até que a variável seja igual a zero. O controle da variável é de inteira responsabilidade do programador e deve ser realizado dentro do bloco.

```
#DATA:
    INT32: a, b, x;
#DATA;

#MAIN:
    SET: a, 64;
    SET: b, 8;
    SET: x, 1;
    // enquanto x não é 0
    #WNZ: x:
        SET: a, SUB(a, b);
        // se a menor ou igual que b
        #INZ: BE(a, b);
            SET: x, 0;
        #INZ;
    #WNZ;
#MAIN;
```

### RUI (repetir até, com incremento)

A operação RUI recebe três parâmetros e possui o objetivo de repetir um trecho de código que se encontra dentro de seu bloco. O primeiro parâmetro obrigatoriamente deve ser uma variável. Este parâmetro é utilizado no controle do laço. O segundo parâmetro pode ser uma variável ou imediato, contanto que seja numérico. Este parâmetro é o valor de parada do laço. Por fim, o terceiro parâmetro, assim como o segundo, pode ser um imediato ou variável numérica, e é utilizado como valor de incremento.

```
#DATA:
    INT32: variavel_de_controle, valor_de_parada, valor_de_incremento;
#DATA;

#MAIN:
    SET: variavel_de_controle, 64;
    SET: valor_de_parada, 0;
    SET: valor_de_incremento, -1;
    #RUI: variavel_de_controle, valor_de_parada, valor_de_incremento:
        // faz alguma coisa
    #RUI;
#MAIN;
```

## Entrada e Saída

Em toda linguagem, é extremamente importante que o usuário possa interagir com o programa, para tal. Faz-se necessário estabelecer as operações básicas de entrada e saída de dados. Na linguagem LIA, temos apenas duas operações básicas de entrada/saída. Sendo elas:

- PRINT - Utilizado para exibir uma variável ou valor imediato na tela.
- SCAN - Utilizado para obter entradas do teclado e armazenar em uma variável.

### PRINT (saída)

A operação **PRINT** tem como objetivo, exibir em tela para o usuário o valor de uma variável ou imediato. Para isto ela pode receber qualquer número de parâmetros (variáveis ou imediatos).

```
PRINT: "Ola me chamo ", nome, " tenho atualmente ", idade, " anos";
```

### SCAN (entrada)

A operação **SCAN** tem como objetivo, obter do teclado do usuário valores que são digitados pelo mesmo. Para isto ela recebe apenas UM parâmetro por vez. Sendo este apenas uma variável.

```
SCAN: x;
```

## Tabela de Tokens

| Token                     | Padrão                                                                                                                                                                                                          |
| ------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `numero`                  | Qualquer sequência de caracteres numéricos que começa com `+`, `-` ou com um caractere numérico e termina com um caractere numérico.                                                                            |
| `string`                  | Qualquer sequência de caracteres alfanuméricos que começa com `"`, em seguida pode possuir uma combinação de caracteres alfanuméricos e alguns símbolos especiais (espaço, `\n`, `\t` e `\0`) e termina em `"`. |
| `caractere`               | Qualquer string que começa com `'`, em seguida possui um caractere alfanumérico ou algum símbolo especial (espaço, `\n`, `\t` e `\0`) e termina em `'`.                                                         |
| `bloc`                    | Qualquer string que contenha os exatos caracteres`BLOC`.                                                                                                                                                        |
| `set`                     | Qualquer string que contenha os exatos caracteres`SET`.                                                                                                                                                         |
| `print`                   | Qualquer string que contenha os exatos caracteres`PRINT`.                                                                                                                                                       |
| `scan`                    | Qualquer string que contenha os exatos caracteres `SCAN`.                                                                                                                                                       |
| `operador`                | Qualquer string formada pelo nome de um operador lógico, relacional ou aritmético.                                                                                                                              |
| `virgula`                 | Qualquer caractere `,`.                                                                                                                                                                                         |
| `ponto_e_virgula`         | Qualquer caractere `;`.                                                                                                                                                                                         |
| `dois_pontos`             | Qualquer caractere `:`.                                                                                                                                                                                         |
| `abre_parenteses`         | Qualquer caractere `(`.                                                                                                                                                                                         |
| `fecha_parenteses`        | Qualquer caractere `)`.                                                                                                                                                                                         |
| `abre_bloco_condicional`  | Qualquer string que começa com `#`, seguido por `INZ`, `WNZ` ou `RUI` e termina em `:`.                                                                                                                         |
| `fecha_bloco_condicional` | Qualquer string que começa com `#`, seguido por `INZ`, `WNZ` ou `RUI` e termina em `;`.                                                                                                                         |
| `abre_bloco_de_codigo`    | Qualquer string que começa com `#`, em seguida possui uma letra maiúscula e, depois, pode possuir uma combinação de caracteres alfanuméricos maiúsculos ou `_` e termina em `:`.                                |
| `fecha_bloco_de_codigo`   | Qualquer string que começa com `#`, em seguida possui uma letra maiúscula e, depois, pode possuir uma combinação de caracteres alfanuméricos maiúsculos ou `_` e termina em `;`.                                |
| `tipo_de_variavel`        | Qualquer string formada pelo nome de um tipo de dado.                                                                                                                                                           |
| `id_de_variavel`          | Qualquer string que começa com letra minúscula, em seguida pode possuir uma combinação de caracteres alfanuméricos ou `_`.                                                                                      |
| `id_de_bloco`             | Qualquer string que começa com letra maiúscula, em seguida pode possuir uma combinação de caracteres alfanuméricos ou `_`.                                                                                      |

## Expressões Regulares

A seguir temos as definições das regras de produção de cada token.

### Números

> - **`numero` -> ( + | - ) ? [ 0 - 9 ]<sup>+</sup>**

### Strings

> - **`string` -> " ( [ a - z ] | [ A - Z ] | [ 0 - 9 ] | \0 | \n | \t | espaço )\*** **"**

### Caractere

> - **`caractere` -> ' ( [ a - z ] | [ A - Z ] | [ 0 - 9 ] | \0 | \n | \t | espaço ) '**

### Palavras reservadas

> - **`bloc` -> BLOC**
> - **`set` -> SET**
> - **`print` -> PRINT**
> - **`scan` -> SCAN**
> - **`operadores` -> DIVR | DIV | MUL | ADD | SUB | NOT | ADD | OR | AE | BE | A | B | E**

### Delimitadores

> - **`virgula` -> ,**
> - **`ponto_e_virgula` -> ;**
> - **`dois_pontos` -> :**
> - **`abre_parenteses` -> (**
> - **`fecha_parenteses` -> )**
> - **`quebra_linha` -> \n**
> - **`abre_bloco_de_codigo` -> # [ A - Z ] ( [ A - Z ] | [ 0 - 9 ] | \_ )\*** **:**
> - **`fecha_bloco_de_codigo` -> # [ A - Z ] ( [ A - Z ] | [ 0 - 9 ] | \_ )\*** **;**
> - **`abre_bloco_condicional` -> # ( INZ | WNZ | RUI ) :**
> - **`fecha_bloco_condicional` -> # ( INZ | WNZ | RUI ) ;**

### Variáveis

> - **`tipos_textuais` -> STR | CHR**
> - **`tipos_numericos` -> INT8 | INT16 | INT32 | INT64 | UINT8 | UINT16 | UINT32 | UINT64**
> - **`id_de_variavel` -> [ a - z ] ( [ a - z ] | [ A - Z ] | [ 0 - 9 ] | \_ )\***

### Bloco

> - **`id_de_bloco` -> [ A - Z ] ( [ A - Z ] | [ 0 - 9 ] | \_ )\***

### Não tokenizadoras

> - **`comentario_de_bloco` -> ! - - ( [ a - z ] | [ A - Z ] | [ 0 - 9 ] | \_ | \n | \t | espaço )\*** **- - !**
> - **`comentario_de_linha` -> / / ( [ a - z ] | [ A - Z ] | [ 0 - 9 ] | \_ | \t | esoaço )\*** **\n**
> - **`irrelevantes` -> ( \t | \n | espaço )<sup>+</sup>**

## GLC

[tipos_variaveis] -> ([tipos_numericos] | [tipos_textuais])

[declaracao_variaveis] -> [tipos_variaveis][dois_pontos][id_de_variavel][ponto_e_virgula]

[parametros] -> [id_de_variaveis] | [cadeia_de_caracteres] | [caractere] | [naturais]

[parametros_numericos] -> [id_de_variaveis] | [naturais]

[parametros_logicos] -> [operacoes_relacional] | [id_de_variaveis] | [naturais]

[parametros_aritmeticos] -> [operadores_numericos] | [operacoes_aritmeticas]

[acima] -> [id][abre_parenteses][id_de_variavel][virgula][parametros_numericos][fecha_parenteses]

[acima_ou_igual] -> ([id])[abre_parenteses][id_de_variavel][virgula][parametros_numericos][fecha_parenteses]

[abaixo] -> [id][abre_parenteses][id_de_variavel][virgula][parametros_numericos][fecha_parenteses]

[abaixo_ou_igual] -> ([id])[abre_parenteses][id_de_variavel][virgula][parametros_numericos][fecha_parenteses]

[igual] -> [id][abre_parenteses][id_de_variavel][virgula][parametros][fecha_parenteses]

[diferente] -> [id][abre_parenteses][id_de_variavel][virgula][parametros][fecha_parenteses]

[atribuicao] -> ([id])[dois_pontos][id_de_variavel][virgula]([parametros] | [operacoes_aritmeticas] | [operacoes_logicas] | [operacoes_relacional]);

[operacoes_relacional] -> [acima] | [acima_ou_igual] | [abaixo] | [abaixo_ou_igual] | [igual] | [diferente]

[e_logico] -> ([id])[abre_paresentes][id_de_variavel][virgula][parametros_logicos][fecha_parenteses]

[ou_logico] -> ([id])[abre_paresentes][id_de_variavel][virgula][parametros_logicos][fecha_parenteses]

[negacao_logica] -> ([id])[abre_paresentes][parametros_logicos][fecha_parenteses]

[operacoes_logicas] -> [e_logico] | [ou_logico] | [negacao_logica]

[soma] -> ([id])[abre_parenteses][id_de_variavel][virgula][parametros_aritmeticos][fecha_parenteses]

[subtracao] -> ([id])[abre_parenteses][id_de_variavel][virgula][parametros_aritmeticos][fecha_parenteses]

[multiplicacao] -> ([id])[abre_parenteses][id_de_variavel][virgula][parametros_aritmeticos][fecha_parenteses]

[divisao] -> ([id])[abre_parenteses][id_de_variavel][virgula][parametros_aritmeticos][fecha_parenteses]

[resto] -> ([id])[abre_parenteses][id_de_variavel][virgula][parametros_aritmeticos][fecha_paresenteses]

[operacoes_aritmeticas] -> [soma] | [subtracao] | [multiplicacao] | [divisao] | [resto]

[inicio_bloco_principal] -> [abre_bloco_de_codigo][quebra_linha]

[fim_bloco_principal] -> [fecha_bloco_de_codigo][quebra_linha]

[inicio_bloco_dados] -> [abre_bloco_de_codigo][quebra_linha]

[fim_bloco_dados] -> [fecha_bloco_de_codigo][quebra_linha]

[inicio_enquanto_nao_zero] -> [abre_bloco_de_codigo][quebra_linha][id_de_variavel][dois_pontos][quebra_linha]

[fim_enquanto_nao_zero] -> [fecha_bloco_de_codigo][quebra_linha]

[inicio_repeticao_com_incremento] -> [abre_bloco_de_codigo][id_de_variavel][virgula][natural][virgula][natural][dois_pontos][quebra_linha]

[fim_repeticao_com_incremento] -> [fecha_bloco_de_codigo][quebra_linha]

[inicio_se_nao_zero] -> [abre_bloco_de_codigo] ([id_de_variavel] | [naturais] | [operacoes_logicas] | [operacoes_relacional])

[fim_se_nao_zero] -> [fecha_bloco_de_codigo][quebra_linha]

[comentarios] -> ([comentario_de_linha] | [comentario_de_bloco])

[entrada] -> [scan][dois_pontos][id_de_variavel][ponto_e_virgula]

[saida] -> [print][dois_pontos][id_de_variavel][ponto_e_virgula]

[inicio] -> [bloco_dados]

[bloco_dados] -> [inicio_bloco_dados][variaveis][fim_bloco_dados][proximo_bloco]

[variaveis] -> [declaracao_variaveis] [variaveis]| [tipos_variaveis] [id_de_variavel][virgula][variaveis]| [id_de_variavel], [variaveis] | [id_de_variavel][ponto_e_virgula]

[proximo_bloco] -> [inicio_de_bloco] [comandos] [fim_de_bloco] [proximo_bloco] | [inicio_de_bloco] [comandos] [fim_de_bloco]

[proximo_bloco] -> [inicio_bloco_principal][comandos] [fim_bloco_principal][proximo_bloco] | [inicio_bloco_principal][comandos][fim_bloco_principal]

[proximo_bloco] -> [comentarios][proximo_bloco] | [irrelevantes]

[comandos] -> [inicio_enquanto_nao_zero][comandos][fim_enquanto_nao_zero]

[comandos] -> [inicio_repeticao_com_incremento][comandos][fim_repeticao_com_incremento]

[comandos] -> [inicio_se_nao_zero][comandos][fim_se_nao_zero]

[comandos] -> [atribuicao][comandos]

[comandos] -> [entrada][comandos] | [saida][comandos]

[comandos] -> [comentarios][comandos] | [comentarios_bloco][comandos]

## BFN

<tipos_variaveis> ::= (<tipos_numericos> | <tipos_textuais>)

<declaracao_variaveis> ::= <tipos_variaveis><dois_pontos><id_de_variavel><ponto_e_virgula>

<parametros> ::= <id_de_variaveis> | <cadeia_de_caracteres> | <caractere> | <naturais>

<parametros_numericos> ::= <id_de_variaveis> | <naturais>

<parametros_logicos> ::= <operacoes_relacional> | <id_de_variaveis> | <naturais>

<parametros_aritmeticos> ::= <operadores_numericos> | <operacoes_aritmeticas>

<acima> ::= <id><abre_parenteses><id_de_variavel><virgula><parametros_numericos><fecha_parenteses>

<acima_ou_igual> ::= (<id>)<abre_parenteses><id_de_variavel><virgula><parametros_numericos><fecha_parenteses>

<abaixo> ::= <id><abre_parenteses><id_de_variavel><virgula><parametros_numericos><fecha_parenteses>

<abaixo_ou_igual> ::= (<id>)<abre_parenteses><id_de_variavel><virgula><parametros_numericos><fecha_parenteses>

<igual> ::= <id><abre_parenteses><id_de_variavel><virgula><parametros><fecha_parenteses>

<diferente> ::= <id><abre_parenteses><id_de_variavel><virgula><parametros><fecha_parenteses>

<atribuicao> ::= (<id>)<dois_pontos><id_de_variavel><virgula>(<parametros> | <operacoes_aritmeticas> | <operacoes_logicas> | <operacoes_relacional>);

<operacoes_relacional> ::= <acima> | <acima_ou_igual> | <abaixo> | <abaixo_ou_igual> | <igual> | <diferente>

<e_logico> ::= (<id>)<abre_paresentes><id_de_variavel><virgula><parametros_logicos><fecha_parenteses>

<ou_logico> ::= (<id>)<abre_paresentes><id_de_variavel><virgula><parametros_logicos><fecha_parenteses>

<negacao_logica> ::= (<id>)<abre_paresentes><parametros_logicos><fecha_parenteses>

<operacoes_logicas> ::= <e_logico> | <ou_logico> | <negacao_logica>

<soma> ::= (<id>)<abre_parenteses><id_de_variavel><virgula><parametros_aritmeticos><fecha_parenteses>

<subtracao> ::= (<id>)<abre_parenteses><id_de_variavel><virgula><parametros_aritmeticos><fecha_parenteses>

<multiplicacao> ::= (<id>)<abre_parenteses><id_de_variavel><virgula><parametros_aritmeticos><fecha_parenteses>

<divisao> ::= (<id>)<abre_parenteses><id_de_variavel><virgula><parametros_aritmeticos><fecha_parenteses>

<resto> ::= (<id>)<abre_parenteses><id_de_variavel><virgula><parametros_aritmeticos><fecha_paresenteses>

<operacoes_aritmeticas> ::= <soma> | <subtracao> | <multiplicacao> | <divisao> | <resto>

<inicio_bloco_principal> ::= <abre_bloco_de_codigo><quebra_linha>

<fim_bloco_principal> ::= <fecha_bloco_de_codigo><quebra_linha>

<inicio_bloco_dados> ::= <abre_bloco_de_codigo><quebra_linha>

<fim_bloco_dados> ::= <fecha_bloco_de_codigo><quebra_linha>

<inicio_enquanto_nao_zero> ::= <abre_bloco_de_codigo><quebra_linha><id_de_variavel><dois_pontos><quebra_linha>

<fim_enquanto_nao_zero> ::= <fecha_bloco_de_codigo><quebra_linha>

<inicio_repeticao_com_incremento> ::= <abre_bloco_de_codigo><id_de_variavel><virgula><natural><virgula><natural><dois_pontos><quebra_linha>

<fim_repeticao_com_incremento> ::= <fecha_bloco_de_codigo><quebra_linha>

<inicio_se_nao_zero> ::= <abre_bloco_de_codigo> (<id_de_variavel> | <naturais> | <operacoes_logicas> | <operacoes_relacional>)

<fim_se_nao_zero> ::= <fecha_bloco_de_codigo><quebra_linha>

<comentarios> ::= (<comentario_de_linha> | <comentario_de_bloco>)

<entrada> ::= <scan><dois_pontos><id_de_variavel><ponto_e_virgula>

<saida> ::= <print><dois_pontos><id_de_variavel><ponto_e_virgula>

<inicio> ::= <bloco_dados>

<bloco_dados> ::= <inicio_bloco_dados><variaveis><fim_bloco_dados><proximo_bloco>

<variaveis> ::= <declaracao_variaveis> <variaveis>`|` <tipos_variaveis> <id_de_variavel><virgula><variaveis>`|` <id_de_variavel>, <variaveis> `|` <id_de_variavel><ponto_e_virgula>

<proximo_bloco> ::= <inicio_de_bloco> <comandos> <fim_de_bloco> <proximo_bloco> `|` <inicio_de_bloco> <comandos> <fim_de_bloco>

<proximo_bloco> ::= <inicio_bloco_principal><comandos> <fim_bloco_principal><proximo_bloco> `|` <inicio_bloco_principal><comandos><fim_bloco_principal>

<proximo_bloco> ::= <comentarios><proximo_bloco> `|` <irrelevantes>

<comandos> ::= <inicio_enquanto_nao_zero><comandos><fim_enquanto_nao_zero>

<comandos> ::= <inicio_repeticao_com_incremento><comandos><fim_repeticao_com_incremento>

<comandos> ::= <inicio_se_nao_zero><comandos><fim_se_nao_zero>

<comandos> ::= <atribuicao><comandos>

<comandos> ::= <entrada><comandos> `|` <saida><comandos>

<comandos> ::= <comentarios><comandos> `|` <comentarios_bloco><comandos>

## Autômatos

### Numerais

![](https://i.imgur.com/rRd19c0.png)

### Caracteres

![](https://i.imgur.com/1NOab3z.png)

### Strings

![](https://i.imgur.com/UOqeSOy.png)

### Nomes de variáveis

![](https://i.imgur.com/7c21TOA.png)

### Comentários em linha

![](https://i.imgur.com/fxzYd1g.png)

### Comentários em bloco

![](https://i.imgur.com/1AHdsyR.png)

### Palavras reservadas & blocos da lingagem

![](https://i.imgur.com/779sjWB.png)

### Automato completo da linaguagem

![](https://i.imgur.com/ZGVHrrz.png)

## Exemplo de código

Nesta sessão, temos os exemplos utilizados como teste no analizador léxico de código.

### Exemplo 1

Obs: gera erro léxico por conta dos caracteres especias do comentário.

```
#DATA:
    INT32: x, y, resultado;
#DATA

#MAIN:
    // resultado = resultado + 2;
    SET: resultado, ADD(resultado, 2);

    // resultado = resultado + (x + (resultado * 2));
    SET: resultado, ADD(resultado, ADD(x, MUL(resultado, 2)));

    BLOC: OPLOGICA;
#MAIN;

#OPLOGICA:
    SET: x, 1;
    SET: y, 0;

    // if (x > y && resultado < y);
    #INZ: AND(A(x, y), B(resultado, y)):
        PRINT: "ola";
    #INZ;
#OPLOGICA;
```

### Exemplo 2

```
#DATA:
    INT8: x, y, zS;
    INT16: a, v_s, zS;
    INT32: x_, y6, zS;
    INT64: xSS, y, zS;
    UINT8: x, y, zS;
    UINT16: x, y, zS;
    UINT32: x, y, zS;
    UINT64: x, y, zS;
    CHR: carac, 'r';
    STR: string, "minha string";
#DATA;

!--
Um bloco de comentario
01
02
--!

#MAIN:
    PRINT: "alguma coisa", x_; // comentario
    SCAN: "alguma coisa", x;
    SET: x, AND(x, OR(x, y));
    BLOC: OPERACAO;
#MAIN;

#OPERACAO:
    #WNZ: NOT(B(zS, 100)):
        SET: zS, MUL(x, y);
    #WNZ;
#OPERACAO;
```

### Exemplo 3

```
#DATA:
    INT32 x, y, resultado;
#DATA;

#MAIN:
    BLOC: SOMA;
#MAIN;

#SOMA:
    SET: resultado, 0;
    SET: resultado, ADD(x, y);
#SOMA;
```

### Exemplo 4

Obs: gera erro léxico por conta dos caracteres especias do comentário.

```
#DATA:
    UINT32: var1, var2, resultado;
    UINT32: soma_var1, soma_var2, soma_resultado;
#DATA;

#MAIN:
    !--
    qualquer modificação em var1 e var2
    --!

    SET: soma_var1, var1;
    SET: soma_var2, var2
    BLOC: SOMA;
    SET: resultado, soma_resultado;

    !--
    qualquer modificação em resultado
    --!
#MAIN:

#SOMA:
    SET: soma_resultado, ADD(soma_var1, soma_var2);
#SOMA;
```
