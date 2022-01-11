## Advent of Code 2021:christmas_tree: in Rust!
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

#### Benchmark (non-scientific) ####
|                  |**Part 1**        |**Part 2**        |
|------------------|-----------------:|-----------------:|
|day 1             |55 us             |66 us             |
|day 2             |98 us             |85 us             |
|day 3             |154 us            |1.23 ms           |
|day 4             |236 us            |413 us            |
|day 5             |1.64 ms           |1.69 ms           |
|day 6             |8 us              |9 us              |
|day 7             |50 us             |20 us             |
|day 8             |97 us             |742 us            |
|day 9             |254 us            |7.02 ms           |
|day 10            |67 us             |118 us            |
|day 11            |292 us            |607 us            |
|day 12            |1.30 ms           |36.31 ms          |
|day 13            |197 us            |528 us            |
|day 14            |206 us            |445 us            |
|day 15            |1.19 ms           |33.33 ms          |
|day 16            |142 us            |142 us            |
|day 17            |4 us              |540 us            |
|day 18            |1.04 ms           |20.04 ms          |
|day 19            |5.59 ms           |5.45 ms           |
|day 20            |191 us            |9.52 ms           |
|day 21            |3 us              |62.54 ms          |
|day 22            |2.68 ms           |6.58 ms           |
|day 23            |25.49 ms          |164.30 ms         |
|day 24            |9 us              |6 us              |
|**Total**         |392.72 ms         |                  |


