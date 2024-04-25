- [x] Merge .tula and .tape files together by implementing `run` command
  - Though, the `run` command may also accept a file path for very
    long tapes
- [x] Blocks of statements
  ```tula
  let Bits { 0 1 }
  for a in Bits {
      case I a 0 -> Next
      case O a 1 -> Next
  }
  ```
- [x] Tracing state vs print it
- [x] Emacs mode
  - If we gonna introduce `run` command, this is already a custom
    keyword that is not present in js.
- [x] Command to expand all the rules.
  - It should basically get rid of all the for loops.
- [x] Case matching based on type checking instead of literal symbol
      substitution.
- [x] Arithmetic operations on values from Integer set
- [x] Error out on the case not using all the variables in the scope
- [x] Type check cases before execusion
- [x] `--no-expr` flag for `expand` subcommand
- [x] Check for unreachable cases
  - Check Sets of Cases overlapping on State and Read
- [x] Union Operations
  ```tula
  for _ in Integer + Bool
  for _ in Integer - Bool
  ```
- [x] Anonymous sets
  ```tule
  for s in {a b c}
  ```
- [x] Fix emoji rendering in the trace
- [ ] Finish off the Eval feature
  - [ ] Forbid in Pattern Matching
  - [x] Forbid in Tapes
  - [x] Forbid in Set Definitions
  - [x] Always Force in Write, Step and Next
- [ ] More Magical Sets
  - [ ] Byte
  - [ ] Real
  - [ ] Boolean
    - Could be user defined
    ```tula
    let Boolean { true false }
    ```
  - [ ] Step
    - Could be user defined
    - Set of all possible Step actions like `->`, `<-`, `.`, `!`, etc
- [ ] Explicitly denote Halt States
  - Useful for catching unreachable states at runtime
- [ ] Something visual, maybe with Raylib
- [ ] Obscure output of `expand` command even more, by replacing states with meaningless words

# Examples

- [x] Fib
- [x] Reverse string
- [x] Balanced parenthesis of different kind
- [x] Universal Turing Machine
- [ ] Brainfuck Interpreter
- [ ] Lambda Calc interpreter

# Indecisive

- [ ] Pattern matching in `for`-loops
  ```tula
  let Invert { (0 1) (1 0) }
  for (a b) in Invert
  case I a b -> I
  ```
  Expands to
  ```tula
  case I 0 1 -> I
  case I 1 0 -> I
  ```
  - I'm indecisive on this one because it's unclear what types `a` and
    `b` should be in this case
- [ ] Magical set `Any`
  - Finite set of all the used expressions in the program.
  - Useful for skipping anything
  - I'm indecisive on this one because with this set it is easy to
    make overlapping cases which we plan to actually forbid
- [ ] Magical set `State`
  - Set of all expressions that are used as State of the problem
  - Useful for defining "Callbacks"
  - Actually such Set might end up recursive
- [x] Sets of sets
  ```tula
  for s in Sets
  for a in s
  ```
  This is basically an easier version of Union Sets
  We've got Union Sets. There is no need for this anymore
