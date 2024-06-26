# Tula

Tula (**Tu**ring **La**nguage) is an [Esoteric Programming Language](https://en.wikipedia.org/wiki/Esoteric_programming_language) based on [Turing Machine](https://en.wikipedia.org/wiki/Turing_machine) and extended with [Set Theory](https://en.wikipedia.org/wiki/Set_theory).

## Install

1. Install [Rust Compiler](https://www.rust-lang.org/learn/get-started)
2. Clone this repo:
```console
$ git clone https://github.com/tsoding/tula && cd tula
```
3. Install the compiler to `~/.cargo/bin/` (it should be already in your `$PATH` after rustup installation)
```console
$ cargo install --path .
```
4. Run some examples:
```console
$ tula run ./examples/05-rule110.tula
```
5. Expand the example into a form without Universal Quantifiers and Sets:
```console
$ tula expand ./examples/05-rule110.tula
```

## Test

The project is using [rere.py](https://github.com/tsoding/rere.py) for testing the behavior of the compiler:

1. To run the tests:
```console
$ ./rere.py replay ./tests.list
```

2. To record the new behavior:
```console
$ ./rere.py record ./tests.list
```

## Base Syntax

The program consist of sequence of rules:

```js
case <State> <Read> <Write> <Step> <Next>
case <State> <Read> <Write> <Step> <Next>
case <State> <Read> <Write> <Step> <Next>
...
```

Each rule starts with the keyword `case` and 5 expressions:

- `<State>` - The current state of the Machine,
- `<Read>` - What the Machine reads on the Tape,
- `<Write>` - What the Machine should write on the Tape,
- `<Step>` - Where the Head of the Machine must step (`<-` left, `->` right or `.` stand),
- `<Next>` - What is the next state of the Machine.

### Example

Simple program that increments a binary number (least significant bits come first):

```js
// When in the state `Inc` and read `0`, replace it with `1` move the head
// to the right and switch to the state `Halt` which halts the program.
case Inc 0 1 -> Halt

// When in the state `Inc` and read `1`, replace it with `0` move the head
// to the right and keep staying in the state `Inc` effectively looping over
// the tape
case Inc 1 0 -> Inc

// Start the program from the state `Inc` with the tape `{ 1 1 0 1 }` and print
// the tape on each state change
trace Inc { 1 1 0 1 }
```

The trace of the execution of the above program:

```
Inc: 1 1 0 1
     ^
Inc: 0 1 0 1
       ^
Inc: 0 0 0 1
         ^
Halt: 0 0 1 1
            ^
```

You can actually have several `trace` statements within a single file
that start at different states with different tapes. All of them are
going to be executed sequentually.

If you need to start the head from a different position provide two
tape sequences. First one is going to be treated as everything to the
left of the head, the second one is everything to the right:
```
trace Loop { a b c } { 1 1 1 0 }
//                     ^
//                     The head starts here
case Loop 1 0 -> Loop
```

The trace of the above program

```
Loop: a b c 1 1 1 0
            ^
Loop: a b c 0 1 1 0
              ^
Loop: a b c 0 0 1 0
                ^
Loop: a b c 0 0 0 0
                  ^
```

## Compound Expressions

Instead of using just Symbols you can actually use Compound Expressions. The syntax of a Compond Expression is similar to [S-expressions](https://en.wikipedia.org/wiki/S-expression) but without the pair syntax. Just Symbols and Lists:

```ebnf
<expr> ::= <symbol>
<expr> ::= "(" *<expr> ")"
```

Here is a simple example that iterates the Tape of Pairs of Numbers and swaps each pair until it reaches the delimiter `&`:

```js
case Swap (1 2) (2 1) -> Swap
case Swap (2 3) (3 2) -> Swap
case Swap (3 4) (4 3) -> Swap

trace Swap { (1 2) (2 3) (3 4) & }
```

Trace of the above program:

```
Swap: (1 2) (2 3) (3 4) &
      ^~~~~
Swap: (2 1) (2 3) (3 4) &
            ^~~~~
Swap: (2 1) (3 2) (3 4) &
                  ^~~~~
Swap: (2 1) (3 2) (4 3) &
                        ^
```

The Compound Expressions don't really add that much to the Language by themselves. We could've written the above program like this and end up with basically this same result:

```js
case Swap 1_2 2_1 -> Swap
case Swap 2_3 3_2 -> Swap
case Swap 3_4 4_3 -> Swap

trace Swap { 1_2 2_3 3_4 & }
```

What they actually do is emphasize that the Symbols may contain additional information and enable use with extrating this information by using Sets and Universal Quantification.

## Sets and Universal Quantification

Tula supports defining Sets (which are collections of Compound Expressions) and using [Universal Quantification](https://en.wikipedia.org/wiki/Universal_quantification) on those Sets to generate Rules automatically.

```js
let Set { a b c }
for n in Set case S n 0 -> S
```

The above program will expand to

```js
case S a 0 -> S
case S b 0 -> S
case S c 0 -> S
```

Note that `for n in Set` quantifier is applied only to a single statement that follows it. To apply the quantifier to a block of statements use curly braces:

```js
let Set { a b c }
for n in Set {
    case S n 0 -> S
    case I n 1 -> I
}
```

The above expands to:

```js
case S a 0 -> S
case I a 1 -> I
case S b 0 -> S
case I b 1 -> I
case S c 0 -> S
case I c 1 -> I
```

You can also nest the Quantifiers:

```js
let Set { a b c }
for n in Set
for m in Set
case (S n) m 0 -> S
```

The above expands to this:

```js
case (S a) a 0 -> S
case (S a) b 0 -> S
case (S a) c 0 -> S
case (S b) a 0 -> S
case (S b) b 0 -> S
case (S b) c 0 -> S
case (S c) a 0 -> S
case (S c) b 0 -> S
case (S c) c 0 -> S
```

Nested Quantifiers that iterate over the same set can be collapsed like so:

```js
let Set { a b c }
for n m in Set
case (S n) m 0 -> S
```

### Example

Here is the example that iterates the Tape of Pairs of Numbers again but using Sets, Universal Quantifiers:

```js
let Numbers { 1 2 3 4 }

// For each `a` and `b` from the set of Numbers when in the state `Swap` and read `(a b)`
// replace it with `(b a)` move the head to the right and stay in the `Swap` state loop
// over the entire tape until you encounter something else
for a b in Numbers
case Swap (a b) (b a) -> Swap

// When in the state `Swap` and read `&`, keep it as `&` move the head to the right and `Halt`
case Swap & & -> Halt

// Execute and trace the program starting from state `Swap` with the tape that contains a
// bunch of pairs of numbers.
trace Swap { (1 2) (2 3) (3 4) & }
```

The trace of the above program:

```
Swap: (1 2) (2 3) (3 4) &
      ^~~~~
Swap: (2 1) (2 3) (3 4) &
            ^~~~~
Swap: (2 1) (3 2) (3 4) &
                  ^~~~~
Swap: (2 1) (3 2) (4 3) &
                        ^
Halt: (2 1) (3 2) (4 3) & &
                          ^
```

The tape is infinite to the left and right filled with the first and last symbols correspondingly. In the example above the tape is filled with `&` to the right, which is clearly indicated by the last trace output.

## Anonymous Sets

It is not necessary to define the Sets upfront with the `let` keyword. You can use them directly in Universal Quantifiers:

```js
for n in { a b c } {
    case S n 0 -> S
}
```

## Set operations

You can combine the Sets with Union and Difference operations (`+` and `-` infix operators correspondingly)

```js
let Numbers { 69 420 }
let Emoji { 😳 🍆 🔥 💯 }

// For any Emoji or Numbers except 🍆 replace it with 🦀.
// This effectively makes the program stop at 🍆 'cause there is no case for it.
for e in Numbers + Emoji - { 🍆 } {
    case Crab e 🦀 -> Crab
}

trace Crab { 🔥 😳 69 420 🍆 }
```

The trace of the above program

```
Crab: 🔥 😳 69 420 🍆
      ^~
Crab: 🦀 😳 69 420 🍆
         ^~
Crab: 🦀 🦀 69 420 🍆
            ^~
Crab: 🦀 🦀 🦀 420 🍆
               ^~~
Crab: 🦀 🦀 🦀 🦀 🍆
                  ^~
```

This kind of Set Expressions are also allowed in the Set Definitions:

```js
let Numbers { 69 420 }
let Emoji { 😳 🍆 🔥 💯 }
let Anything_But_Eggplant ( Numbers + Emoji - { 🍆 } )  // Parenthesis for clarity
let Anything_But_Eggplant Numbers + Emoji - { 🍆 }  // Also works without parenthesis
```

### Cartesian Products

One special operation on the Sets allows you to create [Cartesian Products](https://en.wikipedia.org/wiki/Cartesian_product) of them. Here is how you can Skip all Pairs of Numbers without using nested Universal Quantifiers:

```js
let Number { 1 2 3 4 }
let Pair Number * Number

for _ in Pair
case Skip _ _ -> Skip

trace Skip { (1 2) (2 3) (3 4) & }
```

The trace of the above program:

```
Skip: (1 2) (2 3) (3 4) &
      ^~~~~
Skip: (1 2) (2 3) (3 4) &
            ^~~~~
Skip: (1 2) (2 3) (3 4) &
                  ^~~~~
Skip: (1 2) (2 3) (3 4) &
                        ^
```

## "Magical" Sets

Tula supports a special "magical" set `Integer` that is infinite (actually not, it's `i32`, but you get the point):

```js
for a b in Integer
case Swap (a b) (b a) -> Swap

case Swap & & -> Halt

trace Swap { (69 420) (1337 7331) (42 37) & }
```

The trace of the above program:

```
Swap: (69 420) (1337 7331) (42 37) &
      ^~~~~~~~
Swap: (420 69) (1337 7331) (42 37) &
               ^~~~~~~~~~~
Swap: (420 69) (7331 1337) (42 37) &
                           ^~~~~~~
Swap: (420 69) (7331 1337) (37 42) &
                                   ^
Halt: (420 69) (7331 1337) (37 42) & &
                                     ^
```

It is actually impossible to expand the example because `Integer` is
just too big. But the Interpreter still prints the trace
instantaneously because internally it does not actually generate any
cases. It treats the Sets as Types and performs an efficient Type
Checking and Pattern Matching to infer the `<Write>`, `<Step>` and
`<Next>` based on the current state of the Machine.

You can use `Integer` in Set Expressions:

```js
// For any Integer except 5 specifically keep moving to the right
for n in Integer - { 5 }
case Until_Five n n -> Until_Five
trace Until_Five { 1 2 3 4 5 6 7 8 }
```

Trace of the above program

```
Until_Five: 1 2 3 4 5 6 7 8
            ^
Until_Five: 1 2 3 4 5 6 7 8
              ^
Until_Five: 1 2 3 4 5 6 7 8
                ^
Until_Five: 1 2 3 4 5 6 7 8
                  ^
Until_Five: 1 2 3 4 5 6 7 8
                    ^
```

This specifically makes the program halt at `5` because it does not have a case for it.

Additional Magical Sets include:
- Real - Set of Real Numbers (corresponds to f32 in Rust)
- String - Set of Strings (symbols wrapped in single quotes `'`)

## Eval Expressions (EEs)

Once you've got two Integers the most logical thing to do would be to sum them up:

```js
trace Sum { (1 2) . }
for a b in Integer
case Sum (a b) [a + b] . Halt
```

The trace of the above program:

```
Sum: (1 2) .
     ^~~~~
Halt: 3 .
      ^
```

In the program above `[a + b]` is an Eval Expression (EE). It is a kind of a Compound Expression. Evaluating the Eval Expressions in the Compound Expressions is called Forcing them.

Bellow is a program that fills up the Tape with Fibonacci numbers up until a delimiter `&`:

```js
for a in Integer
case Fib a a -> (Fib a)

for a b in Integer {
    case (Fib a)   b b       -> (Fib a b)
    case (Fib a b) 0 [a + b] .  (Fib b)
}

trace Fib { 0 1 0 0 0 0 & }
```

The trace of the above program:

```
Fib: 0 1 0 0 0 0 &
     ^
(Fib 0): 0 1 0 0 0 0 &
           ^
(Fib 0 1): 0 1 0 0 0 0 &
               ^
(Fib 1): 0 1 1 0 0 0 &
             ^
(Fib 1 1): 0 1 1 0 0 0 &
                 ^
(Fib 1): 0 1 1 2 0 0 &
               ^
(Fib 1 2): 0 1 1 2 0 0 &
                   ^
(Fib 2): 0 1 1 2 3 0 &
                 ^
(Fib 2 3): 0 1 1 2 3 0 &
                     ^
(Fib 3): 0 1 1 2 3 5 &
                   ^
(Fib 3 5): 0 1 1 2 3 5 &
                       ^
```

The syntax of an EE is `"[" <expr> <expr> <expr> "]"` where `<expr>` is a Compound Expression.
- First `<expr>` is the Left-Hand Side operand.
- Second `<expr>` is the operator.
- Third `<expr>` is the Right-Hand Side operand.

Making operands Compound Expressions allows for nesting like this `[[a % 15] == 0]`. Such EEs are Forced Recursively starting from the Inner ones.

Since the operator is also a Compound Expression it is possible to substitute them as well:

```js
let Op { + - }

for a b in Integer
for op in Op
case Eval (a b op) [a op b] -> Eval

trace Eval { (34 35 +) (500 80 -) & }
```

The trace of the above program:

```
Eval: (34 35 +) (500 80 -) &
      ^~~~~~~~~
Eval: 69 (500 80 -) &
         ^~~~~~~~~~
Eval: 69 420 &
             ^
```

- Supported Integer operations: `+`, `-`, `*`, `/`, `%`, `<`, `<=`, `>`, `>=`, `==`, `!=`.
- Supported Real operations: `+`, `-`, `*`, `/`, `%`, `<`, `<=`, `>`, `>=`, `==`, `!=`.
- Supported String operations: `+`, `<`, `<=`, `>`, `>=`, `==`, `!=`.
- Supported Boolean operations: `&&`, `||`, `==`, `!=` (Boolean is either symbol `true` or symbol `false`).
