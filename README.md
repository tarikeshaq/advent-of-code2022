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
1. If it can't find it, it will attempt to read it directly from the advent of code site, and write it back to the file.
1. Parse the input into a UTF-8 string
1. Retrieves a trait object of a [day's solver](src/days/mod.rs)
1. Runs the solver's `q1` and `q2` functions, passing them the parsed input.
1. Prints the answers for both questions to the `stdout`


### Reading inputs from advent-of-code2022
**To be a good citizen, we only attempt to get the input from the advent of code website only once only if we don't have the input locally, and we write the input back once we get it.**

I use a `.env` file that loads up a session cookie, the cookie is then sent with a network request to `https://adventofcode.com/2022/day/<DAY>/input`

To use it
- Rename the [.example.env](.example.env) file to `.env`
- Retrieve your cookie session for adventofcode.com
    - You can do this by navigating to your network tab in your browsers' debug panel then observing the `Cookie` header of the network request. It should be in the form `session=<YOUR SESSION COOKIE>` put your session cookie into the `.env` as the `Cookie` environment variable Make sure to include the `session=`
- If the corresponding `day<DAY>.txt` is empty, the next `cargo run` will automatically go and try to get your input and write it back to the file!


### Where can I see your solution code?
The solution code is available in the [day's directory, there is a file per day](src/days)

### Other
- I also have [a helper script](create_day.sh) that created all the input files and their associated Rust template code.
- I'm trying my best to not use any external dependencies beyond the standard library, I use `tokio`, `reqwest` and `dotenv` to get the input from the adventofcode servers but I avoid dependencies for the solutions.
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
