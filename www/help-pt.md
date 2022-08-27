## Sobre

Esta é uma ferramenta que te permite escrever um programa em Norma2 e compilar para uma
Máquina de Turing.

Um programa em Norma2 é composto por instruções. Cada instrução é formada por um rótulo, uma
operação e uma declaração de saída. As operações envolvem incrementar, decrementar, ou verificar
se um registrador é zero. Em Norma2 (como o nome já sugere), existem apenas dois registradores,
chamados de X e Y. Portanto, temos 3 operações para cada registrador, totalizando 6 operações.

### Exemplo

```
a: if zero X then goto b else goto 0
b: do inc X goto 0
```

A primeira instrução possui o rótulo "a", e é uma operação que verifica se o valor no registrador
X é zero. Se sim, a próxima instrução a ser executada é a com o rótulo "b", senão, o programa para.
Por convenção, quando um rótulo de saída não existe, o programa simplesmente para.

A segunda operação incrementa 1 no registrador X e também para.

### Um exemplo mais elaborado

```
1: if zero X then goto 0 else goto 2
2: do dec X then goto 3
3: do inc Y then goto 4
4: do inc Y then goto 1
```

Todo programa começa com a primeira instrução declarada. Ela testa se X é zero, e para o programa
caso seja. Caso contrário, o programa "executa" as instruções 2, 3, e 4, que juntas, irão decrementar
1 de X, mas incrementar em Y duas vezes.

Ou seja, ao final da execução, o programa pega o valor de X, e adiciona o dobro em Y. Se o par de
valores (X, Y) era (2, 0) no início, o programa finaliza em (0, 4).

### Em Detalhes

Há quatro operações básicas: incX, incY, decX, decY. Elas esperam apenas um rótulo de saída, e podemos
escrever de várias formas:

`do inc Y then goto 0`

` incY    then  goto    0 hisdfusdf`

`incYgoto 0`

Veja que a sintaxe é bem livre (e tudo que vem após o rótulo de saída é ignorado pelo programa). Mas
isso não diminui a rigorosidade do programa, apenas dá mais liberdade na escolha da escrita.

Há duas operações booleanas: zeroX, zeroY. Elas precisam de dois rótulos de saída. A sintaxe, como
já foi visto, não é estricta:

`if zeroX then goto 0 else goto 1`

` zeroX    then  goto    0 else goto 1`

`zeroYgoto 0 elsegoto 1`

Rótulos não precisam ser numeros. Podem ser formados por letras, dígitos, underlines e pontos:

`estes: zeroY goto r0tulos_sao else goto v.a.l.i.d.o.s`

Você pode escrever comentários com `//`. O compilador ignora linhas que comecem com essas
duas barras.

### Salvando a Máquina de Turing

Ao clicar no botão de salvar, o compilador faz download da máquina compilada, com um nome de arquivo
padrão. Você pode alterar o nome do arquivo diretamente no código colocando um comentário na primeira
linha:

```
// arquivo

// ... instruções aqui
```

O arquivo resultante terá o nome `arquivo.mt`