# Advent of Code 2022 🎄🎅

This is my personal repository for [Advent of Code 2022](https://adventofcode.com/)


## Overview
I'm doing this year's advent of code using Rust!

### Usage
You can run the program using:
```sh
$ cargo run <DAY>
```

The program will print the answers to both question for day `DAY` to your `stdout`


### How it's setup
The inputs are in `input/day<DAY>.txt` files and the program is setup as one binary, see [main.rs](src/main.rs).

The binary will:
1. Read the input for the given day from the `input` directory
1. Parse the input into a UTF-8 string
1. Retrieves a trait object of a [day's solver](src/days/mod.rs)
1. Runs the solver's `q1` and `q2` functions, passing them the parsed input.
1. Prints the answers for both questions to the `stdout`


### Where can I see your solution code?
The solution code is available in the [day's directory, there is a file per day](src/days)

### Other
- I also have [a helper script](create_day.sh) that created all the input files and their associated Rust template code.
- I'm trying my best to not use any external dependencies beyond the standard library
- If past years are any indicator, I'll probably stop around halfway but maybe this year is the year I go all the way 🤷
- For my own development, I'm running:
    ```sh
    $ cargo watch -- cargo run <DAY>
    ```
    so the moment I have an answer I can copy it and submit it right away

## Contributions 🤝
This is a personal repository with personal solutions - you are free to fork and contribute (although I can't think of a reason why you'd spend the time to 😆). I'll take contributions that don't change the shape of my solutions and are marginal improvements.


## License
All of the code here is licensed under an MIT license. Go wild if you need to!
