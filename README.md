# RGen

RGen stands for *R*andom *Gen*erator.

## How to run

```sh
# run into repl
$ cargo run
# run into repl while interpreting all lines in a file
$ cargo run test.rgen
```

## Syntax

```odin
// define a generator with a double colon
test :: "define"

// use a vertical bar to seperate possible options
letters :: "a" | "b" | "c"

// reference other generators in a generator
rand_letter :: (generate @letter) | "d"

// use a caret to increase the weight of an option
weighted_generator :: "1"^4 | "2"

// use a period to concat two strings together
concat :: (generate @rand_letter) . (generate @letters) 
```
