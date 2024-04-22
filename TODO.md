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
- [ ] `--no-expr` flag for `expand` subcommand
- [ ] Sets of sets
  ```tula
  for s in Sets
  for a in s
  ```
- [ ] Union sets

# Examples

- [x] Fib
- [ ] Reverse string
- [ ] Brainfuck Interpreter
- [ ] Universal Turing Machine
- [ ] Lambda Calc interpreter

# Low Priority

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
