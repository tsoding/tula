- [ ] Merge .tula and .tape files together by implementing `run` command
  - Though, the `run` command may also accept a file path for very
    long tapes
- [ ] Emacs mode
  - If we gonna introduce `run` command, this is already a custom
    keyword that is not present in js.
- [ ] Pattern matching in `for`-loops
  ```tula
  let Invert { (0 1) (1 0) }
  for (a, b) in Invert
  case I a b -> I
  ```
- [ ] Blocks of statements
  ```tula
  let Bits { 0 1 }
  for a in Bits {
      case I a 0 -> Next
      case O a 1 -> Next
  }
  ```
- [ ] Command to expand all the rules.
  - It should basically get rid of all the for loops.
- [ ] Case matching based on type checking instead of literal symbol
      substitution.
