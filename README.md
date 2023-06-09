# Advent of Code 2019 :christmas_tree: :crab:

My solutions for Advent of Code 2019.

Project structure and setup kindly stolen from [AxlLind](https://github.com/AxlLind) and greatly expanded upon.

## Setup

Create a .env file containing two tokens: YEAR and AOC_SESSION.

AOC_SESSION is your session cookie, acquired from the advent of code website [like this](https://github.com/wimglenn/advent-of-code-wim/issues/1).

## Usage

```sh
just run          # run all days
just run <DAYS>   # run one or more specific days
                  # <DAY> is always two characters, e.g. 01, 25 etc.

just get <DAYS>   # fetch input and create .rs file from template
                  # input_examples/<DAY>.in has to be filled in manually

just post <DAY>   # submit output for automatic checking.
                  # submits part 2 if part 1 has already been submitted
```

## Using example input

The main function of each day needs to be prefixed with an attribute containing the day number, as such:

```rust
#[aoc::main(01)]
```

where the number within the parentheses is the day number. This allows the function to get the input from its corresponding input file in the `/inputs/` folder.

If, however, a second argument is provided, the input will instead be read from the `/input_examples/` folder.

```rust
#[aoc::main(01, 0)]
```

The value of this second argument does not matter, but it needs to be an integer.

## The `phi()` function

The puzzles sometimes require that different variables be used for the example problem and the actual problem, beyond just the input values. The built-in function `phi()` is used to handle such situations.

`phi()` takes two inputs `actual` and `example` of the same type. It returns `actual` when solving the actual problem and `example` when using example input.

## Helper functions

If a function is needed in multiple days, it should be placed in the `helpers` subcrate.
