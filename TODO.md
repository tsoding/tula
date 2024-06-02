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
- [x] Finish off the Eval feature
  - [x] Forbid in Pattern Matching
  - [x] Forbid in Tapes
  - [x] Forbid in Set Definitions
  - [x] Always Force in Write, Step and Next
- [x] Rest of the Operations for Integers and Booleans
- [x] Cartesian Products for Sets
- [x] Custom Integer overflow/underflow Runtime Errors
  - Right now we get a standard Rust panic
- [x] Incosistent double substitution
  - Some ideas to try:
    - [x] Get rid of scoped cases and embrace the repeating rules (you allow them outside of for-loops anyway)
    - [x] Prevent double substitution somehow by marking already substituted expressions?
      - it seems to be naturally happening because of how substitute_bindings work
- [x] Obscure output of `expand` command even more, by replacing states with meaningless words
  - Or even numbers
- [x] Customize the initial location of the head
- [ ] Consistent order of expansion
  - It's actually very hard to enforce because of how set expressions work
  - You can actually sort the expansions
- [ ] More Magical Sets
  - [x] Real
  - [x] String
  - [ ] Boolean
    - Could be user defined
    ```tula
    let Boolean { true false }
    ```
  - [ ] Step
    - Could be user defined
    - Set of all possible Step actions like `->`, `<-`, `.`, `!`, etc
  - [ ] Byte
  - [ ] Char
- [ ] Explicitly denote Halt States
  - Useful for catching unreachable states at runtime

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
  - It's also unclear what to do with the elements of the set that don't
    match the pattern
- [ ] Magical set `Any`
  - Finite set of all the used expressions in the program.
  - Useful for skipping anything
  - I'm indecisive on this one because with this set it is easy to
    make overlapping cases which we plan to actually forbid
- [ ] Magical set `State`
  - Set of all expressions that are used as State of the problem
  - Useful for defining "Callbacks"
  - Actually such Set might end up recursive
- [ ] Subsitutions in Set Expressions
  ```tula
  for s in Sets
  for a in s
  ```
  - This is basically an easier version of Union Sets
  - We've got Union Sets. There is no need for this anymore
  - Actually we may substitute things withing sub expressions
  ```js
  for delim in Delim
  for _ in Bit + Delim - { delim }
  for dir in Dir
  for phase in Phase
  case (Switch delim dir phase) _ _ dir (Switch delim dir phase)
  ```
  - But this is damn hard! You need to first "materialize" delim via the pattern matching, etc, etc
- [ ] Integer sets via ranges
- [ ] Proper infix expressions inside of Evals
  - Since operators themselves also can be substituted I'm not sure how to go about it...
- [ ] Something visual, maybe with Raylib
- [ ] Extension Devices
  - It's unclear how to make this idea usable from the syntactical point of view.
