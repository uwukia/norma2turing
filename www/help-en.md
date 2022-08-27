## About

This is a tool that lets you write a Norma2 program and parses into a Turing file.

A Norma2 program is made up of instructions. Each instruction has its own unique label, an
operation, and a jump declaration. There is only 6 available operations, which operate on only
two registers (or memory locations), X and Y, which are natural numbers (either zero or positive
whole numbers).

It's easier to understand all of that with an example

```
1: if zeroX then goto 2 else goto 0
2: do incX goto 0
```

There are two instructions, labeled "1" and "2". Instruction 1 is a zero instruction, which acts like
an "if zero jump", meaning it'll jump to instruction 2 if the register X is zero. Otherwise, it'll
stop the program. The only reason the program stops, is because we tried to jump to an instruction
that doesn't exist (0), which by default, haults the program.

The operation in instruction 2 (incX) is an increment operation. It'll add 1 to the value in register
X. Meaning, if X was originally 3, it will now be 4. After executing instruction 2, the program
simply haults (since again, instruction 0 is not defined).

Here's a more elaborate program

```
1: if zeroX then goto 0 else goto 2
2: do decX then goto 3
3: do incY then goto 4
4: do incY then goto 1
```

Every program starts with the first instruction, in this case, instruction 1. It checks if X is
zero, and hault the program if so. Otherwise, it jumps to instruction 2. It runs decX, which, as
the name suggests, *decrements* 1 from the register X. Meaning if X was 2, it'll now be equal to 1.
Since we're not allowed to store negative numbers, decrementing from zero does nothing.

You can see it decrements once from X, then increments twice to Y, then loops back to instruction 1.
Meaning the program runs instructions 2, 3, and 4, until X hits zero. Check for yourself that if
X was originally 2, and Y was originally 0, once the program haults, they will be respectively 0 and 4.
Meaning, we took the value from X, and doubled it to Y.

### In-Depth

There are 4 basic operations: incX, incY, decX, decY. Those expect one jump label. When it comes to
syntax, all of these do the same thing:

`do incY then goto 0`

` incY    then  goto    0`

`incYgoto 0`

Syntax is a little bit loose, but allows for flexibility when writing instructions.

There are only two boolean operations: zeroX, zeroY. Those expect two jump labels. The first one
is executed if it's true, second one if false. The syntax, again, can be quite loose.

`if zeroX then goto 0 else goto 1`

` zeroX    then  goto    0 else goto 1`

`zeroYgoto 0 elsegoto 1`

Labels don't have to be numbers. They can be made of letters, digits, underlines or dots.

`these_are: zeroY goto val1d else goto l.a.b.e.l.s`

You are allowed to make line comments with `//`

### Saving Turing Machine

You can retrieve the transpiled Turing Machine from a Norma2 using the save button.

It will use a default filename, but you can change the filename automatically by adding
a comment on the first line:

```
// file

// ... instructions go here
```

This will result in a file called `file.mt`