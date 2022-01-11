/*
 * TODO create CLI for running AOC challenges
 * Provide helper functions for parsing inputs
 */

use std::num::ParseIntError;
use std::time::Duration;

use clap::{App, Arg, ArgMatches};
use clap::{arg};

use aoc_2021;
use aoc_2021::{Part};


fn main() {
    // Create argument parser for framework
    let mut app = App::new("Advent of Code Template for Rust.")
        .version("V0.1")
        .author("Jelle M. <https://github.com/Jellycious>")
        .subcommand(
            App::new("desc")
            .about("Gets description for day")
            .arg(arg!([day] "Day number").required(true)))
        .subcommand(
            App::new("solve")
            .about("Tries to solve a specific day")
            .arg(arg!([day] "Day number").required(true))
            .arg(Arg::new("bench")
                .short('b')
                .help("Print Time Taken")))
        .subcommand(
            App::new("solve-all")
            .about("Tries to solve all days sequentially")
            .arg(Arg::new("print")
                .short('p')
                .help("Print solutions"))
            .arg(Arg::new("bench")
                .short('b')
                .help("Print Time Taken")))
        .subcommand(
            App::new("input")
            .about("Retrieves the input for a day")
            .arg(arg!([day] "Day number").required(true))
            .arg(Arg::new("store")
                .short('s')
                .help("Store the input as a file in inputs directory")))
        .subcommand(
            App::new("bench")
            .about("Benchmarks all of the puzzles (non-scientifically)"));

    let parser = app.get_matches_mut();

    // check which subcommand
    match parser.subcommand() {
        Some(("desc", sub_m)) => { description(sub_m) },
        Some(("solve", sub_m)) => { solve(sub_m)},
        Some(("solve-all", sub_m)) => { solve_all(sub_m)},
        Some(("input", sub_m)) => { input(sub_m)},
        Some(("bench", _)) => { benchmark()},
        _ => { 
            eprintln!("Invalid Command, provide -h for help"); 
            app.print_help().unwrap();
        },
    }

}

/// Solves all puzzles sequentially
fn solve_all(_matches: &ArgMatches) {
    let print_solution = _matches.is_present("print");
    let print_dur = _matches.is_present("bench");
    let mut part1_dur = Duration::new(0,0);
    let mut part2_dur = Duration::new(0,0);
    // iterate through days
    for i in 1..26 {
        println!("{}", aoc_2021::get_day_header(i));
        let solution_1 = aoc_2021::solve(i, Part::One);
        if solution_1.is_none() {
            println!("- Part 1: ❌");
        }else {
            let (sol, dur) = solution_1.unwrap();
            part1_dur += dur;

            let time_info;
            if print_dur {
                time_info = format!(" ({})", duration_to_string(dur));
            }else {
                time_info = String::from("");
            }

            println!("- Part 1: ✅{}", time_info);

            if print_solution { println!("\tSolution: {}", sol); }
        }

        let solution_2 = aoc_2021::solve(i, Part::Two);
        if solution_2.is_none() {
            println!("- Part 2: ❌");
        }else {
            let (sol, dur) = solution_2.unwrap();
            part2_dur += dur;

            let time_info;
            if print_dur {
                time_info = format!(" ({})", duration_to_string(dur));
            }else {
                time_info = String::from("");
            }

            println!("- Part 2: ✅{}", time_info);

            if print_solution { println!("\tSolution: {}", sol); }
        }
    }

    if print_dur {
        println!("");
        println!("### Part 1: {}", duration_to_string(part1_dur));
        println!("### Part 2: {}", duration_to_string(part2_dur));
        println!("### Total: {}", duration_to_string(part1_dur + part2_dur));
    }
}

fn benchmark() {
    print!("## Benchmark\n\n");
    println!("|{:18}|{:18}|{:18}|","","**Part 1**","**Part 2**");
    println!("|{:-<18}|{:-<17}:|{:-<17}:|","","","");
    let mut total_dur = Duration::new(0,0);
    for i in 1..25 {
        print!("|day {:<14}", i);
        let solution1 = aoc_2021::solve(i, Part::One);
        if solution1.is_none() {
            print!("|{:18}", "");
        }else {
            let dur = solution1.unwrap().1;
            total_dur+=dur;
            print!("|{:18}", duration_to_string(dur));
        }
        let solution2 = aoc_2021::solve(i, Part::Two);
        if solution2.is_none() {
            print!("|{:18}|", "");
        }else {
            let dur = solution2.unwrap().1;
            total_dur+=dur;
            print!("|{:18}|\n", duration_to_string(dur));
        }
    }
    println!("|{:18}|{:18}|{:18}|", "**Total**", duration_to_string(total_dur), "");

}

fn duration_to_string(dur: Duration) -> String {
    let us = dur.as_micros();
    if us < 1000 { // us
        format!("{} us", us)
    }else if us < 1000000 { // ms
        let us = us as f64;
        format!("{:.2} ms", us / 1000.0)
    }else { // second
        let us = us as f64;
        format!("{:.2} s", us / 1000000.0)
    }
}

/// Gets description for puzzle
fn description(matches: &ArgMatches) {
    let day= retrieve_day_arg(matches);
    if day.is_none() {
        return;
    }
    let num = day.unwrap();
    let desc = aoc_2021::get_day_description(num);
    println!("{}", desc);
}

/// Solves the puzzle for a specific day
fn solve(matches: &ArgMatches) {
    let print_dur = matches.is_present("bench");

    let day = retrieve_day_arg(matches);
    if day.is_none() {
        return;
    }
    let num = day.unwrap();
    let solution_1 = aoc_2021::solve(num, Part::One);
    if solution_1.is_none() {
        println!("Part 1 for Day {} has not been solved yet.", num);
        return;
    }

    let (sol, dur) = solution_1.unwrap();
    let dur_str;
    if print_dur {
        dur_str = format!(" ({})", duration_to_string(dur));
    }else {
        dur_str = String::from("");

    }
    println!("Part 1{}:\n{}", dur_str, sol);

    let solution_2 = aoc_2021::solve(num, Part::Two);
    if solution_2.is_none() {
        println!("Part 2 for Day {} has not been solved yet.", num);
        return;
    }

    let (sol, dur) = solution_2.unwrap();
    let dur_str;
    if print_dur {
        dur_str = format!(" ({})", duration_to_string(dur));
    }else {
        dur_str = String::from("");

    }

    println!("Part 2:{}\n{}", dur_str, sol);
}

/// Retrieves the input for a puzzle
fn input(matches: &ArgMatches) {
    let day = retrieve_day_arg(matches);
    if day.is_none() {
        return;
    }
    let num = day.unwrap();
    let input = aoc_2021::get_day_input(num);
    print!("{}", input);
}

// ===== helper functions =====
/// Retrieves the day number argument from user input
fn retrieve_day_arg(matches: &ArgMatches) -> Option<u32> {
    let day = matches.value_of("day");
    if day.is_none() {
        //eprintln!("{}\n", matches.usage());
        eprintln!("Please provide a day");
        return None
    }
    let num_res: Result<u32, ParseIntError> = day.unwrap().parse();
    match num_res {
        Ok(num) => {
            if num < 1 || num > 25 {
                //eprintln!("{}\n", matches.usage());
                eprintln!("You must provide a number between 1 and 25");
                return None
            }
            Some(num)
        },
        Err(_) => {
            //eprintln!("{}\n", matches.usage());
            eprintln!("You must provide a number between 1 and 25");
            return None
        }
    }
}

