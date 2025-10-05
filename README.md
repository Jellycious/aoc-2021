## Advent of Code 2021 🎄 in Rust!
This repository contains my [Advent of Code](https://adventofcode.com/) 2021 Solutions for Rust.
Runs **all** solutions **under half a second** on an AMD Ryzen 7 3700X (16) @ 3.600G processor!

Last year I completed Advent of Code (AOC) in [Haskell](https://github.com/Jellycious/aoc-2020). 
I really enjoyed the experience and decided to attempt this year as well.
This time getting my feet wet with [Rust](https://www.rust-lang.org/):crab:. 
Up to this point I was not really comfortable with Rust yet.
I attempted to create a fast (somewhat elegant) solution for every problem, without relying to heavily upon existing external crates.
At some points my code became pretty verbose and I think this is something I could improve upon.
Yet all in all I am really happy with the result and definitely learnt a great deal.

#### Running the solution:
I created somewhat of a framework to retrieve input files and descriptions from the advent of code website. Feel free to use it for your own Advent of Code solutions. See `cargo run -- --help` for the list of possible commands.
 
**Running all solutions:**
```
cargo run --release -- solve-all -b
```

**Benchmarking all solutions:**
```
cargo run --release -- bench
```

#### Benchmark

|                  |**Part 1**        |**Part 2**        |
|------------------|-----------------:|-----------------:|
|day 1             |51 us             |58 us             |
|day 2             |92 us             |78 us             |
|day 3             |146 us            |1.30 ms           |
|day 4             |244 us            |424 us            |
|day 5             |1.70 ms           |1.73 ms           |
|day 6             |7 us              |9 us              |
|day 7             |52 us             |21 us             |
|day 8             |105 us            |762 us            |
|day 9             |259 us            |10.77 ms          |
|day 10            |69 us             |120 us            |
|day 11            |299 us            |619 us            |
|day 12            |1.34 ms           |34.43 ms          |
|day 13            |129 us            |353 us            |
|day 14            |136 us            |441 us            |
|day 15            |1.24 ms           |34.06 ms          |
|day 16            |141 us            |139 us            |
|day 17            |4 us              |554 us            |
|day 18            |1.25 ms           |20.51 ms          |
|day 19            |5.62 ms           |5.42 ms           |
|day 20            |193 us            |9.72 ms           |
|day 21            |3 us              |64.88 ms          |
|day 22            |2.99 ms           |6.78 ms           |
|day 23            |26.08 ms          |167.32 ms         |
|day 24            |9 us              |7 us              |
|day 25            |36.35 ms          |0 us              |
|**Total**         |439.04 ms         |                  |
