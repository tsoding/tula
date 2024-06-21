:i count 22
:b shell 39
cargo run -q run ./examples/01-inc.tula
:i returncode 0
:b stdout 486
./examples/01-inc.tula:7:1: trace
Inc: 0 0 0 1 0
     ^
Halt: 1 0 0 1 0
        ^
./examples/01-inc.tula:8:1: trace
Inc: 1 1 1 0
     ^
Inc: 0 1 1 0
       ^
Inc: 0 0 1 0
         ^
Inc: 0 0 0 0
           ^
Halt: 0 0 0 1 0
              ^
./examples/01-inc.tula:10:1: trace
Dec: 0 0 0 1 0
     ^
Dec: 1 0 0 1 0
       ^
Dec: 1 1 0 1 0
         ^
Dec: 1 1 1 1 0
           ^
Halt: 1 1 1 0 0
              ^
./examples/01-inc.tula:11:1: trace
Dec: 1 1 1 0
     ^
Halt: 0 1 1 0
        ^

:b stderr 0

:b shell 42
cargo run -q run ./examples/02-parens.tula
:i returncode 0
:b stdout 5555
./examples/02-parens.tula:4:1: trace
Entry: & '(' '(' '(' ')' ')' ')' & 0
       ^
Pick: & '(' '(' '(' ')' ')' ')' & 0
        ^~~
(Pick '('): & & '(' '(' ')' ')' ')' & 0
                ^~~
(Pick '('): & & '(' '(' ')' ')' ')' & 0
                    ^~~
(Pick '('): & & '(' '(' ')' ')' ')' & 0
                        ^~~
(Pick '('): & & '(' '(' ')' ')' ')' & 0
                            ^~~
(Pick '('): & & '(' '(' ')' ')' ')' & 0
                                ^~~
(Pick '('): & & '(' '(' ')' ')' ')' & 0
                                    ^
Inc: & & '(' '(' ')' ')' ')' & 0
                               ^
(Reset Bits): & & '(' '(' ')' ')' ')' & 1
                                      ^
(Reset Parens): & & '(' '(' ')' ')' ')' & 1
                                    ^~~
(Reset Parens): & & '(' '(' ')' ')' ')' & 1
                                ^~~
(Reset Parens): & & '(' '(' ')' ')' ')' & 1
                            ^~~
(Reset Parens): & & '(' '(' ')' ')' ')' & 1
                        ^~~
(Reset Parens): & & '(' '(' ')' ')' ')' & 1
                    ^~~
(Reset Parens): & & '(' '(' ')' ')' ')' & 1
                  ^
Pick: & & '(' '(' ')' ')' ')' & 1
          ^~~
(Pick '('): & & & '(' ')' ')' ')' & 1
                  ^~~
(Pick '('): & & & '(' ')' ')' ')' & 1
                      ^~~
(Pick '('): & & & '(' ')' ')' ')' & 1
                          ^~~
(Pick '('): & & & '(' ')' ')' ')' & 1
                              ^~~
(Pick '('): & & & '(' ')' ')' ')' & 1
                                  ^
Inc: & & & '(' ')' ')' ')' & 1
                             ^
Inc: & & & '(' ')' ')' ')' & 1 0
                               ^
(Reset Bits): & & & '(' ')' ')' ')' & 1 1
                                      ^
(Reset Bits): & & & '(' ')' ')' ')' & 1 1
                                    ^
(Reset Parens): & & & '(' ')' ')' ')' & 1 1
                                  ^~~
(Reset Parens): & & & '(' ')' ')' ')' & 1 1
                              ^~~
(Reset Parens): & & & '(' ')' ')' ')' & 1 1
                          ^~~
(Reset Parens): & & & '(' ')' ')' ')' & 1 1
                      ^~~
(Reset Parens): & & & '(' ')' ')' ')' & 1 1
                    ^
Pick: & & & '(' ')' ')' ')' & 1 1
            ^~~
(Pick '('): & & & & ')' ')' ')' & 1 1
                    ^~~
(Pick '('): & & & & ')' ')' ')' & 1 1
                        ^~~
(Pick '('): & & & & ')' ')' ')' & 1 1
                            ^~~
(Pick '('): & & & & ')' ')' ')' & 1 1
                                ^
Inc: & & & & ')' ')' ')' & 1 1
                           ^
Inc: & & & & ')' ')' ')' & 1 1
                             ^
Inc: & & & & ')' ')' ')' & 1 1 0
                               ^
(Reset Bits): & & & & ')' ')' ')' & 1 1 1
                                      ^
(Reset Bits): & & & & ')' ')' ')' & 1 1 1
                                    ^
(Reset Bits): & & & & ')' ')' ')' & 1 1 1
                                  ^
(Reset Parens): & & & & ')' ')' ')' & 1 1 1
                                ^~~
(Reset Parens): & & & & ')' ')' ')' & 1 1 1
                            ^~~
(Reset Parens): & & & & ')' ')' ')' & 1 1 1
                        ^~~
(Reset Parens): & & & & ')' ')' ')' & 1 1 1
                      ^
Pick: & & & & ')' ')' ')' & 1 1 1
              ^~~
(Pick ')'): & & & & & ')' ')' & 1 1 1
                      ^~~
(Pick ')'): & & & & & ')' ')' & 1 1 1
                          ^~~
(Pick ')'): & & & & & ')' ')' & 1 1 1
                              ^
Dec: & & & & & ')' ')' & 1 1 1
                         ^
Dec: & & & & & ')' ')' & 1 1 1
                           ^
Dec: & & & & & ')' ')' & 1 1 1
                             ^
Dec: & & & & & ')' ')' & 1 1 1 0
                               ^
Dec1: & & & & & ')' ')' & 1 1 1 0
                              ^
(Reset Bits): & & & & & ')' ')' & 1 1 0 0
                                    ^
(Reset Bits): & & & & & ')' ')' & 1 1 0 0
                                  ^
(Reset Bits): & & & & & ')' ')' & 1 1 0 0
                                ^
(Reset Parens): & & & & & ')' ')' & 1 1 0 0
                              ^~~
(Reset Parens): & & & & & ')' ')' & 1 1 0 0
                          ^~~
(Reset Parens): & & & & & ')' ')' & 1 1 0 0
                        ^
Pick: & & & & & ')' ')' & 1 1 0 0
                ^~~
(Pick ')'): & & & & & & ')' & 1 1 0 0
                        ^~~
(Pick ')'): & & & & & & ')' & 1 1 0 0
                            ^
Dec: & & & & & & ')' & 1 1 0 0
                       ^
Dec: & & & & & & ')' & 1 1 0 0
                         ^
Dec: & & & & & & ')' & 1 1 0 0
                           ^
Dec1: & & & & & & ')' & 1 1 0 0
                          ^
(Reset Bits): & & & & & & ')' & 1 0 0 0
                                ^
(Reset Bits): & & & & & & ')' & 1 0 0 0
                              ^
(Reset Parens): & & & & & & ')' & 1 0 0 0
                            ^~~
(Reset Parens): & & & & & & ')' & 1 0 0 0
                          ^
Pick: & & & & & & ')' & 1 0 0 0
                  ^~~
(Pick ')'): & & & & & & & & 1 0 0 0
                          ^
Dec: & & & & & & & & 1 0 0 0
                     ^
Dec: & & & & & & & & 1 0 0 0
                       ^
Dec1: & & & & & & & & 1 0 0 0
                      ^
(Reset Bits): & & & & & & & & 0 0 0 0
                            ^
(Reset Parens): & & & & & & & & 0 0 0 0
                            ^
Pick: & & & & & & & & 0 0 0 0
                    ^
Verify: & & & & & & & & 0 0 0 0
                        ^
Balanced: & & & & & & & & 0 0 0 0
                            ^

:b stderr 0

:b shell 39
cargo run -q run ./examples/02-add.tula
:i returncode 0
:b stdout 3166
./examples/02-add.tula:1:1: trace
Add: % 1 1 0 0 & 0 1 0 0
     ^
Dec: % 1 1 0 0 & 0 1 0 0
       ^
(Switch & -> Inc): % 0 1 0 0 & 0 1 0 0
                       ^
(Switch & -> Inc): % 0 1 0 0 & 0 1 0 0
                         ^
(Switch & -> Inc): % 0 1 0 0 & 0 1 0 0
                           ^
(Switch & -> Inc): % 0 1 0 0 & 0 1 0 0
                             ^
Inc: % 0 1 0 0 & 0 1 0 0
                 ^
(Switch % <- Dec): % 0 1 0 0 & 1 1 0 0
                                 ^
(Switch % <- Dec): % 0 1 0 0 & 1 1 0 0
                               ^
(Switch % <- Dec): % 0 1 0 0 & 1 1 0 0
                             ^
(Switch % <- Dec): % 0 1 0 0 & 1 1 0 0
                           ^
(Switch % <- Dec): % 0 1 0 0 & 1 1 0 0
                         ^
(Switch % <- Dec): % 0 1 0 0 & 1 1 0 0
                       ^
(Switch % <- Dec): % 0 1 0 0 & 1 1 0 0
                     ^
(Switch % <- Dec): % 0 1 0 0 & 1 1 0 0
                   ^
Dec: % 0 1 0 0 & 1 1 0 0
       ^
Dec: % 1 1 0 0 & 1 1 0 0
         ^
(Switch & -> Inc): % 1 0 0 0 & 1 1 0 0
                         ^
(Switch & -> Inc): % 1 0 0 0 & 1 1 0 0
                           ^
(Switch & -> Inc): % 1 0 0 0 & 1 1 0 0
                             ^
Inc: % 1 0 0 0 & 1 1 0 0
                 ^
Inc: % 1 0 0 0 & 0 1 0 0
                   ^
Inc: % 1 0 0 0 & 0 0 0 0
                     ^
(Switch % <- Dec): % 1 0 0 0 & 0 0 1 0
                                     ^
(Switch % <- Dec): % 1 0 0 0 & 0 0 1 0
                                   ^
(Switch % <- Dec): % 1 0 0 0 & 0 0 1 0
                                 ^
(Switch % <- Dec): % 1 0 0 0 & 0 0 1 0
                               ^
(Switch % <- Dec): % 1 0 0 0 & 0 0 1 0
                             ^
(Switch % <- Dec): % 1 0 0 0 & 0 0 1 0
                           ^
(Switch % <- Dec): % 1 0 0 0 & 0 0 1 0
                         ^
(Switch % <- Dec): % 1 0 0 0 & 0 0 1 0
                       ^
(Switch % <- Dec): % 1 0 0 0 & 0 0 1 0
                     ^
(Switch % <- Dec): % 1 0 0 0 & 0 0 1 0
                   ^
Dec: % 1 0 0 0 & 0 0 1 0
       ^
(Switch & -> Inc): % 0 0 0 0 & 0 0 1 0
                       ^
(Switch & -> Inc): % 0 0 0 0 & 0 0 1 0
                         ^
(Switch & -> Inc): % 0 0 0 0 & 0 0 1 0
                           ^
(Switch & -> Inc): % 0 0 0 0 & 0 0 1 0
                             ^
Inc: % 0 0 0 0 & 0 0 1 0
                 ^
(Switch % <- Dec): % 0 0 0 0 & 1 0 1 0
                                 ^
(Switch % <- Dec): % 0 0 0 0 & 1 0 1 0
                               ^
(Switch % <- Dec): % 0 0 0 0 & 1 0 1 0
                             ^
(Switch % <- Dec): % 0 0 0 0 & 1 0 1 0
                           ^
(Switch % <- Dec): % 0 0 0 0 & 1 0 1 0
                         ^
(Switch % <- Dec): % 0 0 0 0 & 1 0 1 0
                       ^
(Switch % <- Dec): % 0 0 0 0 & 1 0 1 0
                     ^
(Switch % <- Dec): % 0 0 0 0 & 1 0 1 0
                   ^
Dec: % 0 0 0 0 & 1 0 1 0
       ^
Dec: % 1 0 0 0 & 1 0 1 0
         ^
Dec: % 1 1 0 0 & 1 0 1 0
           ^
Dec: % 1 1 1 0 & 1 0 1 0
             ^
Dec: % 1 1 1 1 & 1 0 1 0
               ^
Done: % 1 1 1 1 & 1 0 1 0
                ^

:b stderr 0

:b shell 41
cargo run -q run ./examples/03-pairs.tula
:i returncode 0
:b stdout 376
./examples/03-pairs.tula:5:1: trace
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

:b stderr 0

:b shell 43
cargo run -q run ./examples/05-rule110.tula
:i returncode 0
:b stdout 1041
./examples/05-rule110.tula:3:1: run
% . . . . . . . . . . . . . . . # . & 1 1 1 1 1 1 1 1 1 1 1 1 1 0 
% . . . . . . . . . . . . . . # # . & 1 1 1 1 1 1 1 1 1 1 1 1 1 0 
% . . . . . . . . . . . . . # # # . & 1 1 1 1 1 1 1 1 1 1 1 1 0 0 
% . . . . . . . . . . . . # # . # . & 1 1 1 1 1 1 1 1 1 1 1 0 0 0 
% . . . . . . . . . . . # # # # # . & 1 1 1 1 1 1 1 1 1 1 0 0 0 0 
% . . . . . . . . . . # # . . . # . & 1 1 1 1 1 1 1 1 1 0 0 0 0 0 
% . . . . . . . . . # # # . . # # . & 1 1 1 1 1 1 1 1 0 0 0 0 0 0 
% . . . . . . . . # # . # . # # # . & 1 1 1 1 1 1 1 0 0 0 0 0 0 0 
% . . . . . . . # # # # # # # . # . & 1 1 1 1 1 1 0 0 0 0 0 0 0 0 
% . . . . . . # # . . . . . # # # . & 1 1 1 1 1 0 0 0 0 0 0 0 0 0 
% . . . . . # # # . . . . # # . # . & 1 1 1 1 0 0 0 0 0 0 0 0 0 0 
% . . . . # # . # . . . # # # # # . & 1 1 1 0 0 0 0 0 0 0 0 0 0 0 
% . . . # # # # # . . # # . . . # . & 1 1 0 0 0 0 0 0 0 0 0 0 0 0 
% . . # # . . . # . # # # . . # # . & 1 0 0 0 0 0 0 0 0 0 0 0 0 0 
% . # # # . . # # # # . # . # # # . & 0 0 0 0 0 0 0 0 0 0 0 0 0 0 

:b stderr 0

:b shell 41
cargo run -q run ./examples/06-shift.tula
:i returncode 0
:b stdout 80
./examples/06-shift.tula:3:1: run
& 69 420 1337 1 2 3 & 
& 420 1337 1 2 3 69 & 

:b stderr 0

:b shell 39
cargo run -q run ./examples/07-fib.tula
:i returncode 0
:b stdout 3982
./examples/07-fib.tula:9:1: trace
Fib: 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 &
     ^
(Fib 0): 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 &
           ^
(Fib 0 1): 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 &
               ^
(Fib 1): 0 1 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 &
             ^
(Fib 1 1): 0 1 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 &
                 ^
(Fib 1): 0 1 1 2 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 &
               ^
(Fib 1 2): 0 1 1 2 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 &
                   ^
(Fib 2): 0 1 1 2 3 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 &
                 ^
(Fib 2 3): 0 1 1 2 3 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 &
                     ^
(Fib 3): 0 1 1 2 3 5 0 0 0 0 0 0 0 0 0 0 0 0 0 0 &
                   ^
(Fib 3 5): 0 1 1 2 3 5 0 0 0 0 0 0 0 0 0 0 0 0 0 0 &
                       ^
(Fib 5): 0 1 1 2 3 5 8 0 0 0 0 0 0 0 0 0 0 0 0 0 &
                     ^
(Fib 5 8): 0 1 1 2 3 5 8 0 0 0 0 0 0 0 0 0 0 0 0 0 &
                         ^
(Fib 8): 0 1 1 2 3 5 8 13 0 0 0 0 0 0 0 0 0 0 0 0 &
                       ^~
(Fib 8 13): 0 1 1 2 3 5 8 13 0 0 0 0 0 0 0 0 0 0 0 0 &
                             ^
(Fib 13): 0 1 1 2 3 5 8 13 21 0 0 0 0 0 0 0 0 0 0 0 &
                           ^~
(Fib 13 21): 0 1 1 2 3 5 8 13 21 0 0 0 0 0 0 0 0 0 0 0 &
                                 ^
(Fib 21): 0 1 1 2 3 5 8 13 21 34 0 0 0 0 0 0 0 0 0 0 &
                              ^~
(Fib 21 34): 0 1 1 2 3 5 8 13 21 34 0 0 0 0 0 0 0 0 0 0 &
                                    ^
(Fib 34): 0 1 1 2 3 5 8 13 21 34 55 0 0 0 0 0 0 0 0 0 &
                                 ^~
(Fib 34 55): 0 1 1 2 3 5 8 13 21 34 55 0 0 0 0 0 0 0 0 0 &
                                       ^
(Fib 55): 0 1 1 2 3 5 8 13 21 34 55 89 0 0 0 0 0 0 0 0 &
                                    ^~
(Fib 55 89): 0 1 1 2 3 5 8 13 21 34 55 89 0 0 0 0 0 0 0 0 &
                                          ^
(Fib 89): 0 1 1 2 3 5 8 13 21 34 55 89 144 0 0 0 0 0 0 0 &
                                       ^~~
(Fib 89 144): 0 1 1 2 3 5 8 13 21 34 55 89 144 0 0 0 0 0 0 0 &
                                               ^
(Fib 144): 0 1 1 2 3 5 8 13 21 34 55 89 144 233 0 0 0 0 0 0 &
                                            ^~~
(Fib 144 233): 0 1 1 2 3 5 8 13 21 34 55 89 144 233 0 0 0 0 0 0 &
                                                    ^
(Fib 233): 0 1 1 2 3 5 8 13 21 34 55 89 144 233 377 0 0 0 0 0 &
                                                ^~~
(Fib 233 377): 0 1 1 2 3 5 8 13 21 34 55 89 144 233 377 0 0 0 0 0 &
                                                        ^
(Fib 377): 0 1 1 2 3 5 8 13 21 34 55 89 144 233 377 610 0 0 0 0 &
                                                    ^~~
(Fib 377 610): 0 1 1 2 3 5 8 13 21 34 55 89 144 233 377 610 0 0 0 0 &
                                                            ^
(Fib 610): 0 1 1 2 3 5 8 13 21 34 55 89 144 233 377 610 987 0 0 0 &
                                                        ^~~
(Fib 610 987): 0 1 1 2 3 5 8 13 21 34 55 89 144 233 377 610 987 0 0 0 &
                                                                ^
(Fib 987): 0 1 1 2 3 5 8 13 21 34 55 89 144 233 377 610 987 1597 0 0 &
                                                            ^~~~
(Fib 987 1597): 0 1 1 2 3 5 8 13 21 34 55 89 144 233 377 610 987 1597 0 0 &
                                                                      ^
(Fib 1597): 0 1 1 2 3 5 8 13 21 34 55 89 144 233 377 610 987 1597 2584 0 &
                                                                  ^~~~
(Fib 1597 2584): 0 1 1 2 3 5 8 13 21 34 55 89 144 233 377 610 987 1597 2584 0 &
                                                                            ^
(Fib 2584): 0 1 1 2 3 5 8 13 21 34 55 89 144 233 377 610 987 1597 2584 4181 &
                                                                       ^~~~
(Fib 2584 4181): 0 1 1 2 3 5 8 13 21 34 55 89 144 233 377 610 987 1597 2584 4181 &
                                                                                 ^

:b stderr 0

:b shell 43
cargo run -q run ./examples/08-reverse.tula
:i returncode 0
:b stdout 5380
./examples/08-reverse.tula:3:1: trace
Entry: % a b c a b c & _
       ^
Init: % a b c a b c & _
        ^
Init: % a b c a b c & _
          ^
Init: % a b c a b c & _
            ^
Init: % a b c a b c & _
              ^
Init: % a b c a b c & _
                ^
Init: % a b c a b c & _
                  ^
Init: % a b c a b c & _
                    ^
Pick: % a b c a b c & _
                  ^
(Pick c): % a b c a b & & _
                        ^
(Pick c): % a b c a b & & _
                          ^
Reset: % a b c a b & & c
                     ^
Pick: % a b c a b & & c
                  ^
Pick: % a b c a b & & c
                ^
(Pick b): % a b c a & & & c
                      ^
(Pick b): % a b c a & & & c
                        ^
(Pick b): % a b c a & & & c
                          ^
(Pick b): % a b c a & & & c _
                            ^
Reset: % a b c a & & & c b
                       ^
Reset: % a b c a & & & c b
                     ^
Pick: % a b c a & & & c b
                  ^
Pick: % a b c a & & & c b
                ^
Pick: % a b c a & & & c b
              ^
(Pick a): % a b c & & & & c b
                    ^
(Pick a): % a b c & & & & c b
                      ^
(Pick a): % a b c & & & & c b
                        ^
(Pick a): % a b c & & & & c b
                          ^
(Pick a): % a b c & & & & c b
                            ^
(Pick a): % a b c & & & & c b _
                              ^
Reset: % a b c & & & & c b a
                         ^
Reset: % a b c & & & & c b a
                       ^
Reset: % a b c & & & & c b a
                     ^
Pick: % a b c & & & & c b a
                  ^
Pick: % a b c & & & & c b a
                ^
Pick: % a b c & & & & c b a
              ^
Pick: % a b c & & & & c b a
            ^
(Pick c): % a b & & & & & c b a
                  ^
(Pick c): % a b & & & & & c b a
                    ^
(Pick c): % a b & & & & & c b a
                      ^
(Pick c): % a b & & & & & c b a
                        ^
(Pick c): % a b & & & & & c b a
                          ^
(Pick c): % a b & & & & & c b a
                            ^
(Pick c): % a b & & & & & c b a
                              ^
(Pick c): % a b & & & & & c b a _
                                ^
Reset: % a b & & & & & c b a c
                           ^
Reset: % a b & & & & & c b a c
                         ^
Reset: % a b & & & & & c b a c
                       ^
Reset: % a b & & & & & c b a c
                     ^
Pick: % a b & & & & & c b a c
                  ^
Pick: % a b & & & & & c b a c
                ^
Pick: % a b & & & & & c b a c
              ^
Pick: % a b & & & & & c b a c
            ^
Pick: % a b & & & & & c b a c
          ^
(Pick b): % a & & & & & & c b a c
                ^
(Pick b): % a & & & & & & c b a c
                  ^
(Pick b): % a & & & & & & c b a c
                    ^
(Pick b): % a & & & & & & c b a c
                      ^
(Pick b): % a & & & & & & c b a c
                        ^
(Pick b): % a & & & & & & c b a c
                          ^
(Pick b): % a & & & & & & c b a c
                            ^
(Pick b): % a & & & & & & c b a c
                              ^
(Pick b): % a & & & & & & c b a c
                                ^
(Pick b): % a & & & & & & c b a c _
                                  ^
Reset: % a & & & & & & c b a c b
                             ^
Reset: % a & & & & & & c b a c b
                           ^
Reset: % a & & & & & & c b a c b
                         ^
Reset: % a & & & & & & c b a c b
                       ^
Reset: % a & & & & & & c b a c b
                     ^
Pick: % a & & & & & & c b a c b
                  ^
Pick: % a & & & & & & c b a c b
                ^
Pick: % a & & & & & & c b a c b
              ^
Pick: % a & & & & & & c b a c b
            ^
Pick: % a & & & & & & c b a c b
          ^
Pick: % a & & & & & & c b a c b
        ^
(Pick a): % & & & & & & & c b a c b
              ^
(Pick a): % & & & & & & & c b a c b
                ^
(Pick a): % & & & & & & & c b a c b
                  ^
(Pick a): % & & & & & & & c b a c b
                    ^
(Pick a): % & & & & & & & c b a c b
                      ^
(Pick a): % & & & & & & & c b a c b
                        ^
(Pick a): % & & & & & & & c b a c b
                          ^
(Pick a): % & & & & & & & c b a c b
                            ^
(Pick a): % & & & & & & & c b a c b
                              ^
(Pick a): % & & & & & & & c b a c b
                                ^
(Pick a): % & & & & & & & c b a c b
                                  ^
(Pick a): % & & & & & & & c b a c b _
                                    ^
Reset: % & & & & & & & c b a c b a
                               ^
Reset: % & & & & & & & c b a c b a
                             ^
Reset: % & & & & & & & c b a c b a
                           ^
Reset: % & & & & & & & c b a c b a
                         ^
Reset: % & & & & & & & c b a c b a
                       ^
Reset: % & & & & & & & c b a c b a
                     ^
Pick: % & & & & & & & c b a c b a
                  ^
Pick: % & & & & & & & c b a c b a
                ^
Pick: % & & & & & & & c b a c b a
              ^
Pick: % & & & & & & & c b a c b a
            ^
Pick: % & & & & & & & c b a c b a
          ^
Pick: % & & & & & & & c b a c b a
        ^
Pick: % & & & & & & & c b a c b a
      ^

:b stderr 0

:b shell 48
cargo run -q run ./examples/09-multi-parens.tula
:i returncode 0
:b stdout 4865
./examples/09-multi-parens.tula:5:1: trace
Entry: % '[' '(' ')' ']' '{' '}' & .
       ^
Pick: % '[' '(' ')' ']' '{' '}' & .
        ^~~
(Pick '['): % % '(' ')' ']' '{' '}' & .
                ^~~
(Pick '['): % % '(' ')' ']' '{' '}' & .
                    ^~~
(Pick '['): % % '(' ')' ']' '{' '}' & .
                        ^~~
(Pick '['): % % '(' ')' ']' '{' '}' & .
                            ^~~
(Pick '['): % % '(' ')' ']' '{' '}' & .
                                ^~~
(Pick '['): % % '(' ')' ']' '{' '}' & .
                                    ^
(Update '['): % % '(' ')' ']' '{' '}' & .
                                        ^
Entry: % % '(' ')' ']' '{' '}' & '['
                               ^
Entry: % % '(' ')' ']' '{' '}' & '['
                           ^~~
Entry: % % '(' ')' ']' '{' '}' & '['
                       ^~~
Entry: % % '(' ')' ']' '{' '}' & '['
                   ^~~
Entry: % % '(' ')' ']' '{' '}' & '['
               ^~~
Entry: % % '(' ')' ']' '{' '}' & '['
           ^~~
Entry: % % '(' ')' ']' '{' '}' & '['
         ^
Pick: % % '(' ')' ']' '{' '}' & '['
          ^~~
(Pick '('): % % % ')' ']' '{' '}' & '['
                  ^~~
(Pick '('): % % % ')' ']' '{' '}' & '['
                      ^~~
(Pick '('): % % % ')' ']' '{' '}' & '['
                          ^~~
(Pick '('): % % % ')' ']' '{' '}' & '['
                              ^~~
(Pick '('): % % % ')' ']' '{' '}' & '['
                                  ^
(Update '('): % % % ')' ']' '{' '}' & '['
                                      ^~~
(Update '('): % % % ')' ']' '{' '}' & '[' .
                                          ^
Entry: % % % ')' ']' '{' '}' & '[' '('
                               ^~~
Entry: % % % ')' ']' '{' '}' & '[' '('
                             ^
Entry: % % % ')' ']' '{' '}' & '[' '('
                         ^~~
Entry: % % % ')' ']' '{' '}' & '[' '('
                     ^~~
Entry: % % % ')' ']' '{' '}' & '[' '('
                 ^~~
Entry: % % % ')' ']' '{' '}' & '[' '('
             ^~~
Entry: % % % ')' ']' '{' '}' & '[' '('
           ^
Pick: % % % ')' ']' '{' '}' & '[' '('
            ^~~
(Pick ')'): % % % % ']' '{' '}' & '[' '('
                    ^~~
(Pick ')'): % % % % ']' '{' '}' & '[' '('
                        ^~~
(Pick ')'): % % % % ']' '{' '}' & '[' '('
                            ^~~
(Pick ')'): % % % % ']' '{' '}' & '[' '('
                                ^
(Update ')'): % % % % ']' '{' '}' & '[' '('
                                    ^~~
(Update ')'): % % % % ']' '{' '}' & '[' '('
                                        ^~~
(Update ')'): % % % % ']' '{' '}' & '[' '(' .
                                            ^
(Pop ')'): % % % % ']' '{' '}' & '[' '(' .
                                     ^~~
Entry: % % % % ']' '{' '}' & '[' . .
                             ^~~
Entry: % % % % ']' '{' '}' & '[' . .
                           ^
Entry: % % % % ']' '{' '}' & '[' . .
                       ^~~
Entry: % % % % ']' '{' '}' & '[' . .
                   ^~~
Entry: % % % % ']' '{' '}' & '[' . .
               ^~~
Entry: % % % % ']' '{' '}' & '[' . .
             ^
Pick: % % % % ']' '{' '}' & '[' . .
              ^~~
(Pick ']'): % % % % % '{' '}' & '[' . .
                      ^~~
(Pick ']'): % % % % % '{' '}' & '[' . .
                          ^~~
(Pick ']'): % % % % % '{' '}' & '[' . .
                              ^
(Update ']'): % % % % % '{' '}' & '[' . .
                                  ^~~
(Update ']'): % % % % % '{' '}' & '[' . .
                                      ^
(Pop ']'): % % % % % '{' '}' & '[' . .
                               ^~~
Entry: % % % % % '{' '}' & . . .
                         ^
Entry: % % % % % '{' '}' & . . .
                     ^~~
Entry: % % % % % '{' '}' & . . .
                 ^~~
Entry: % % % % % '{' '}' & . . .
               ^
Pick: % % % % % '{' '}' & . . .
                ^~~
(Pick '{'): % % % % % % '}' & . . .
                        ^~~
(Pick '{'): % % % % % % '}' & . . .
                            ^
(Update '{'): % % % % % % '}' & . . .
                                ^
Entry: % % % % % % '}' & '{' . .
                       ^
Entry: % % % % % % '}' & '{' . .
                   ^~~
Entry: % % % % % % '}' & '{' . .
                 ^
Pick: % % % % % % '}' & '{' . .
                  ^~~
(Pick '}'): % % % % % % % & '{' . .
                          ^
(Update '}'): % % % % % % % & '{' . .
                              ^~~
(Update '}'): % % % % % % % & '{' . .
                                  ^
(Pop '}'): % % % % % % % & '{' . .
                           ^~~
Entry: % % % % % % % & . . .
                     ^
Entry: % % % % % % % & . . .
                   ^
Pick: % % % % % % % & . . .
                    ^
Verify: % % % % % % % & . . .
                        ^
Balanced: % % % % % % % & . . .
                          ^

:b stderr 0

:b shell 45
cargo run -q run ./examples/10-fizz-buzz.tula
:i returncode 0
:b stdout 454
./examples/10-fizz-buzz.tula:1:1: run
1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz 16 17 Fizz 19 Buzz Fizz 22 23 Fizz Buzz 26 Fizz 28 29 FizzBuzz 31 32 Fizz 34 Buzz Fizz 37 38 Fizz Buzz 41 Fizz 43 44 FizzBuzz 46 47 Fizz 49 Buzz Fizz 52 53 Fizz Buzz 56 Fizz 58 59 FizzBuzz 61 62 Fizz 64 Buzz Fizz 67 68 Fizz Buzz 71 Fizz 73 74 FizzBuzz 76 77 Fizz 79 Buzz Fizz 82 83 Fizz Buzz 86 Fizz 88 89 FizzBuzz 91 92 Fizz 94 Buzz Fizz 97 98 Fizz Buzz . 

:b stderr 0

:b shell 39
cargo run -q run ./examples/11-utm.tula
:i returncode 0
:b stdout 9001
./examples/11-utm.tula:10:1: trace
(UTM Inc): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & (head 1) 1 0 1 0
           ^
(UTM Inc): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & (head 1) 1 0 1 0
             ^~~~~~~~~~~~~~~~
(UTM Inc): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & (head 1) 1 0 1 0
                              ^~~~~~~~~~~~~~~~
(UTM Inc): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & (head 1) 1 0 1 0
                                               ^~~~~~~~~~~~~~~~
(UTM Inc): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & (head 1) 1 0 1 0
                                                                ^~~~~~~~~~~~~~~~
(UTM Inc): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & (head 1) 1 0 1 0
                                                                                 ^
(UTM Inc): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & (head 1) 1 0 1 0
                                                                                   ^~~~~~~~
(Match Inc 1): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & (head 1) 1 0 1 0
                                                                                     ^
(Match Inc 1): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & (head 1) 1 0 1 0
                                                                    ^~~~~~~~~~~~~~~~
(Match Inc 1): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & (head 1) 1 0 1 0
                                                   ^~~~~~~~~~~~~~~~
(Match Inc 1): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & (head 1) 1 0 1 0
                                  ^~~~~~~~~~~~~~~~
(Write 0 -> Inc): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & (head 1) 1 0 1 0
                                                      ^~~~~~~~~~~~~~~~
(Write 0 -> Inc): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & (head 1) 1 0 1 0
                                                                       ^~~~~~~~~~~~~~~~
(Write 0 -> Inc): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & (head 1) 1 0 1 0
                                                                                        ^
(Write 0 -> Inc): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & (head 1) 1 0 1 0
                                                                                          ^~~~~~~~
(Next Inc): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & 0 1 0 1 0
                                                                                      ^
(Match Inc 1): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & 0 (head 1) 0 1 0
                                                                                       ^
(Match Inc 1): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & 0 (head 1) 0 1 0
                                                                                     ^
(Match Inc 1): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & 0 (head 1) 0 1 0
                                                                    ^~~~~~~~~~~~~~~~
(Match Inc 1): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & 0 (head 1) 0 1 0
                                                   ^~~~~~~~~~~~~~~~
(Match Inc 1): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & 0 (head 1) 0 1 0
                                  ^~~~~~~~~~~~~~~~
(Write 0 -> Inc): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & 0 (head 1) 0 1 0
                                                      ^~~~~~~~~~~~~~~~
(Write 0 -> Inc): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & 0 (head 1) 0 1 0
                                                                       ^~~~~~~~~~~~~~~~
(Write 0 -> Inc): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & 0 (head 1) 0 1 0
                                                                                        ^
(Write 0 -> Inc): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & 0 (head 1) 0 1 0
                                                                                          ^
(Write 0 -> Inc): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & 0 (head 1) 0 1 0
                                                                                            ^~~~~~~~
(Next Inc): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & 0 0 0 1 0
                                                                                        ^
(Match Inc 0): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & 0 0 (head 0) 1 0
                                                                                         ^
(Match Inc 0): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & 0 0 (head 0) 1 0
                                                                                       ^
(Match Inc 0): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & 0 0 (head 0) 1 0
                                                                                     ^
(Match Inc 0): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & 0 0 (head 0) 1 0
                                                                    ^~~~~~~~~~~~~~~~
(Match Inc 0): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & 0 0 (head 0) 1 0
                                                   ^~~~~~~~~~~~~~~~
(Match Inc 0): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & 0 0 (head 0) 1 0
                                  ^~~~~~~~~~~~~~~~
(Match Inc 0): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & 0 0 (head 0) 1 0
                 ^~~~~~~~~~~~~~~~
(Write 1 . Halt): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & 0 0 (head 0) 1 0
                                     ^~~~~~~~~~~~~~~~
(Write 1 . Halt): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & 0 0 (head 0) 1 0
                                                      ^~~~~~~~~~~~~~~~
(Write 1 . Halt): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & 0 0 (head 0) 1 0
                                                                       ^~~~~~~~~~~~~~~~
(Write 1 . Halt): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & 0 0 (head 0) 1 0
                                                                                        ^
(Write 1 . Halt): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & 0 0 (head 0) 1 0
                                                                                          ^
(Write 1 . Halt): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & 0 0 (head 0) 1 0
                                                                                            ^
(Write 1 . Halt): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & 0 0 (head 0) 1 0
                                                                                              ^~~~~~~~
(Next Halt): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & 0 0 1 1 0
                                                                                         ^
(Match Halt 1): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & 0 0 (head 1) 1 0
                                                                                          ^
(Match Halt 1): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & 0 0 (head 1) 1 0
                                                                                        ^
(Match Halt 1): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & 0 0 (head 1) 1 0
                                                                                      ^
(Match Halt 1): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & 0 0 (head 1) 1 0
                                                                     ^~~~~~~~~~~~~~~~
(Match Halt 1): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & 0 0 (head 1) 1 0
                                                    ^~~~~~~~~~~~~~~~
(Match Halt 1): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & 0 0 (head 1) 1 0
                                   ^~~~~~~~~~~~~~~~
(Match Halt 1): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & 0 0 (head 1) 1 0
                  ^~~~~~~~~~~~~~~~
(Match Halt 1): % (Inc 0 1 . Halt) (Inc 1 0 -> Inc) (Dec 1 0 . Halt) (Dec 0 1 -> Dec) & 0 0 (head 1) 1 0
                ^

:b stderr 0

:b shell 47
cargo run -q run ./examples/12-bubble-sort.tula
:i returncode 0
:b stdout 4038
./examples/12-bubble-sort.tula:1:1: trace
Bubble_Sort: . 2 6 4 1 5 3 .
             ^
Loop: . 2 6 4 1 5 3 .
        ^
(Loop 2): . 2 6 4 1 5 3 .
              ^
(Swap? false 2 6): . 2 6 4 1 5 3 .
                       ^
(Loop 6): . 2 6 4 1 5 3 .
                ^
(Swap? true 6 4): . 2 6 4 1 5 3 .
                        ^
(Swap1 6 4): . 2 6 6 1 5 3 .
                 ^
(Swap2 6 4): . 2 4 6 1 5 3 .
                   ^
(Loop 6): . 2 4 6 1 5 3 .
                  ^
(Swap? true 6 1): . 2 4 6 1 5 3 .
                          ^
(Swap1 6 1): . 2 4 6 6 5 3 .
                   ^
(Swap2 6 1): . 2 4 1 6 5 3 .
                     ^
(Loop 6): . 2 4 1 6 5 3 .
                    ^
(Swap? true 6 5): . 2 4 1 6 5 3 .
                            ^
(Swap1 6 5): . 2 4 1 6 6 3 .
                     ^
(Swap2 6 5): . 2 4 1 5 6 3 .
                       ^
(Loop 6): . 2 4 1 5 6 3 .
                      ^
(Swap? true 6 3): . 2 4 1 5 6 3 .
                              ^
(Swap1 6 3): . 2 4 1 5 6 6 .
                       ^
(Swap2 6 3): . 2 4 1 5 3 6 .
                         ^
(Loop 6): . 2 4 1 5 3 6 .
                        ^
(Narrow 6): . 2 4 1 5 3 6 6
                        ^
Reset: . 2 4 1 5 3 . 6
                 ^
Reset: . 2 4 1 5 3 . 6
               ^
Reset: . 2 4 1 5 3 . 6
             ^
Reset: . 2 4 1 5 3 . 6
           ^
Reset: . 2 4 1 5 3 . 6
         ^
Reset: . 2 4 1 5 3 . 6
       ^
Loop: . 2 4 1 5 3 . 6
        ^
(Loop 2): . 2 4 1 5 3 . 6
              ^
(Swap? false 2 4): . 2 4 1 5 3 . 6
                       ^
(Loop 4): . 2 4 1 5 3 . 6
                ^
(Swap? true 4 1): . 2 4 1 5 3 . 6
                        ^
(Swap1 4 1): . 2 4 4 5 3 . 6
                 ^
(Swap2 4 1): . 2 1 4 5 3 . 6
                   ^
(Loop 4): . 2 1 4 5 3 . 6
                  ^
(Swap? false 4 5): . 2 1 4 5 3 . 6
                           ^
(Loop 5): . 2 1 4 5 3 . 6
                    ^
(Swap? true 5 3): . 2 1 4 5 3 . 6
                            ^
(Swap1 5 3): . 2 1 4 5 5 . 6
                     ^
(Swap2 5 3): . 2 1 4 3 5 . 6
                       ^
(Loop 5): . 2 1 4 3 5 . 6
                      ^
(Narrow 5): . 2 1 4 3 5 5 6
                      ^
Reset: . 2 1 4 3 . 5 6
               ^
Reset: . 2 1 4 3 . 5 6
             ^
Reset: . 2 1 4 3 . 5 6
           ^
Reset: . 2 1 4 3 . 5 6
         ^
Reset: . 2 1 4 3 . 5 6
       ^
Loop: . 2 1 4 3 . 5 6
        ^
(Loop 2): . 2 1 4 3 . 5 6
              ^
(Swap? true 2 1): . 2 1 4 3 . 5 6
                      ^
(Swap1 2 1): . 2 2 4 3 . 5 6
               ^
(Swap2 2 1): . 1 2 4 3 . 5 6
                 ^
(Loop 2): . 1 2 4 3 . 5 6
                ^
(Swap? false 2 4): . 1 2 4 3 . 5 6
                         ^
(Loop 4): . 1 2 4 3 . 5 6
                  ^
(Swap? true 4 3): . 1 2 4 3 . 5 6
                          ^
(Swap1 4 3): . 1 2 4 4 . 5 6
                   ^
(Swap2 4 3): . 1 2 3 4 . 5 6
                     ^
(Loop 4): . 1 2 3 4 . 5 6
                    ^
(Narrow 4): . 1 2 3 4 4 5 6
                    ^
Reset: . 1 2 3 . 4 5 6
             ^
Reset: . 1 2 3 . 4 5 6
           ^
Reset: . 1 2 3 . 4 5 6
         ^
Reset: . 1 2 3 . 4 5 6
       ^
Loop: . 1 2 3 . 4 5 6
        ^
(Loop 1): . 1 2 3 . 4 5 6
              ^
(Swap? false 1 2): . 1 2 3 . 4 5 6
                       ^
(Loop 2): . 1 2 3 . 4 5 6
                ^
(Swap? false 2 3): . 1 2 3 . 4 5 6
                         ^
(Loop 3): . 1 2 3 . 4 5 6
                  ^
(Narrow 3): . 1 2 3 3 4 5 6
                  ^
Reset: . 1 2 . 3 4 5 6
           ^
Reset: . 1 2 . 3 4 5 6
         ^
Reset: . 1 2 . 3 4 5 6
       ^
Loop: . 1 2 . 3 4 5 6
        ^
(Loop 1): . 1 2 . 3 4 5 6
              ^
(Swap? false 1 2): . 1 2 . 3 4 5 6
                       ^
(Loop 2): . 1 2 . 3 4 5 6
                ^
(Narrow 2): . 1 2 2 3 4 5 6
                ^
Reset: . 1 . 2 3 4 5 6
         ^
Reset: . 1 . 2 3 4 5 6
       ^
Loop: . 1 . 2 3 4 5 6
        ^
(Loop 1): . 1 . 2 3 4 5 6
              ^
(Narrow 1): . 1 1 2 3 4 5 6
              ^
Reset: . . 1 2 3 4 5 6
       ^
Loop: . . 1 2 3 4 5 6
        ^

:b stderr 0

:b shell 36
cargo run -q run ./examples/bb2.tula
:i returncode 0
:b stdout 141
./examples/bb2.tula:1:1: trace
A: 0
   ^
B: 1 0
     ^
A: 1 1
   ^
B: 0 1 1
   ^
A: 0 1 1 1
   ^
B: 1 1 1 1
     ^
Halt: 1 1 1 1
          ^

:b stderr 0

:b shell 40
cargo run -q run ./euler/problem-01.tula
:i returncode 0
:b stdout 41
./euler/problem-01.tula:5:1: run
233168 

:b stderr 0

:b shell 40
cargo run -q run ./euler/problem-02.tula
:i returncode 0
:b stdout 42
./euler/problem-02.tula:2:1: run
4613732 

:b stderr 0

:b shell 40
cargo run -q run ./euler/problem-03.tula
:i returncode 0
:b stdout 53
./euler/problem-03.tula:4:1: run
71 839 1471 6857 0 

:b stderr 0

:b shell 34
cargo run -q run ./tests/real.tula
:i returncode 0
:b stdout 189
./tests/real.tula:1:1: trace
Sum_Real: 3.4 3.5 .
          ^~~
(Sum_Real 3.4): . 3.5 .
                  ^~~
(Sum_Real 3.4 3.5): . . .
                        ^
Halt: . . 6.9
          ^~~

:b stderr 0

:b shell 36
cargo run -q run ./tests/string.tula
:i returncode 0
:b stdout 251
./tests/string.tula:1:1: trace
Hello: 'Hello, ' 'World' .
       ^~~~~~~~~
(Hello 'Hello, '): . 'World' .
                     ^~~~~~~
(Hello 'Hello, ' 'World'): . . .
                               ^
Halt: . . 'Hello, World'
          ^~~~~~~~~~~~~~

:b stderr 0

:b shell 39
cargo run -q run ./tests/left-tape.tula
:i returncode 0
:b stdout 68
./tests/left-tape.tula:1:1: trace
Left: 0
      ^
Halt: 0 1
      ^

:b stderr 0

:b shell 45
cargo run -q expand ./tests/double-subst.tula
:i returncode 0
:b stdout 23
case (b 69) . . . Halt

:b stderr 0

:b shell 41
cargo run -q run ./tests/unused-vars.tula
:i returncode 1
:b stdout 0

:b stderr 210
./tests/unused-vars.tula:4:5: ERROR: not all variables in the scope are used in the input of the case
./tests/unused-vars.tula:3:5: NOTE: unused variable a
./tests/unused-vars.tula:3:7: NOTE: unused variable b

:b shell 41
cargo run -q run ./tests/custom-head.tula
:i returncode 0
:b stdout 144
./tests/custom-head.tula:1:1: trace
Entry: 0 1 2 3 4 .
             ^
Entry: 0 1 2 4 4 .
               ^
Entry: 0 1 2 4 5 .
                 ^

:b stderr 0

