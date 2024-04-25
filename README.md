# Tula

**Tu**ring **La**nguage. An [Esoteric Programming Language](https://en.wikipedia.org/wiki/Esoteric_programming_language) based on [Turing Machine](https://en.wikipedia.org/wiki/Turing_machine) extended with [Set Theory](https://en.wikipedia.org/wiki/Set_theory), Compound Expressions and [Pattern Matching](https://en.wikipedia.org/wiki/Pattern_matching).

*The Language is currently in Development. So the Source Code is not available yet. The Development is happening at https://twitch.tv/tsoding The Source Code will be available as soon as I feel like the project is ready. Also I'll be making a detailed Video about this Language on my YouTube channel https://youtube.com/@Tsoding*

## Base Syntax

The program consist of sequence of rules:

```js
case <State> <Read> <Write> <Step> <Next>
case <State> <Read> <Write> <Step> <Next>
case <State> <Read> <Write> <Step> <Next>
...
```

- `<State>` - The current state of the Machine,
- `<Read>` - What the Machine reads on the Tape,
- `<Write>` - What the Machine should write on the Tape,
- `<Step>` - Where the Head of the Machine must step (`<-` left or `->` right),
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

## Compound Expressions

Instead of using just Symbols you can actually use Compound Expressions. Here is a simple example that iterates the Tape of Pairs of Numbers and swaps each pair until it reaches the delimiter `&`:

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
```

The Compound Expressions don't really add that much to the Language by themselves. We could've written the above program like this and end up with basically this same result:

```js
case Swap 1_2 2_1 -> Swap
case Swap 2_3 3_2 -> Swap
case Swap 3_4 4_3 -> Swap
case Swap & & -> Halt

trace Swap { 1_2 2_3 3_4 & }
```

What they actually do is emphasize that the Symbols may contain additional information and enable use with extrating this information through Pattern Matching.

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
case (S a) a a -> S
case (S a) b a -> S
case (S a) c a -> S
case (S b) a b -> S
case (S b) b b -> S
case (S b) c b -> S
case (S c) a c -> S
case (S c) b c -> S
case (S c) c c -> S
```

Nested Quantifiers that iterate over the same set can be collapsed like so:

```js
let Set { a b c }
for n m in Set
case (S n) m 0 -> S
```

### Example

Here is the example that iterates the Tape of Pairs of Numbers again but using Sets, Universal Quantifiers and Pattern Matching:

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

The tape is infinite to the right (but not the left!) and filled with the last symbol. In the example above it's `&`.

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
let Emoji { 😳 🍆 🔥 💯 }

// For any Emoji or Integer except 🍆 replace it with 🦀.
// This effectively makes the program stop at 🍆 'cause there is no case for it.
for e in Integer + Emoji - { 🍆 } {
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
let Emoji { 😳 🍆 🔥 💯 }
let Anything_But_Eggplant ( Integer + Emoji - { 🍆 } )  // Parenthesis for clarity
let Anything_But_Eggplant Integer + Emoji - { 🍆 }  // Also works without parenthesis
```

## "Magical" Sets

Tula supports a special "magical" set `Integer` that is infinite
(actually not, it's `i32`, but you get the point):

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
