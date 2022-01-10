use crate::AOCDay;

use std::collections::HashMap;
use std::hash::{Hash};

use itertools::Itertools;

/*
 * Day 21: Dirac Dice
 *
 * Play a game of dirac dice with a special 100-sided deterministic dice
 */

pub struct Day21();

impl AOCDay for Day21 {
    fn part1(&self, _input: &str) -> Option<String> { Some(part1(_input)) }
    fn part2(&self, _input: &str) -> Option<String> { Some(part2(_input)) }
    fn get_num(&self) -> u32 { 21 }
}

pub fn get() -> Day21 {Day21()}

#[derive(Debug)]
struct DetDice(u32, u32); //prev, rolls

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
struct Player(u32, u32); //pos, score

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
struct Universe {
    player1: Player,
    player2: Player
}

impl DetDice {
    fn roll(&mut self) -> u32 {
        self.0 = self.0 % 100 + 1;
        self.1 += 1;
        self.0
    }
    fn new() -> Self {DetDice(0, 0)}
}

impl Iterator for DetDice {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.roll())
    }
}

fn part1(input: &str) -> String {
    let (mut p1, mut p2) = parse(&input);
    let mut dice = DetDice::new();
    let mut player_one_turn = true;

    while p1.1 < 1000 && p2.1 < 1000 {
        let roll = dice.by_ref().take(3).sum();
        if player_one_turn {
            play_turn_det(&mut p1, roll);
        }else {
            play_turn_det(&mut p2, roll);
        }
        player_one_turn = !player_one_turn;
    }

    let loser;
    if p1.1 >= 1000 {
        loser = p2;
    }else {
        loser = p1;
    }
    format!("{}", dice.1 * loser.1)
}

fn part2(input: &str) -> String {
    let (p1, p2) = parse(&input);

    let mut ended_games = HashMap::new();
    let mut universes = HashMap::new();
    let universe = Universe{player1: p1, player2: p2};
    universes.insert(universe, 1);

    let mut p1_turn = true;
    while universes.len() > 0 {
        universes = play_turn_quantum(&universes, p1_turn, &mut ended_games);
        p1_turn = !p1_turn;
    }
    format!("{}", ended_games.values().max().unwrap())
}

/// Plays a non-deterministic turn with Dirac Dice, keeps track of the number of universes, which
/// are the same.
fn play_turn_quantum(universes: &HashMap<Universe, u64>, p1_turn: bool, ended_games: &mut HashMap<bool, u64>) -> HashMap<Universe, u64> {
    let mut new_universes = HashMap::new();
    for (universe, count) in universes.into_iter() {
        // Every universe branches into 27 new universes.
        let rolls: Vec<u32> = dirac_throw().into_iter()
            .cartesian_product(dirac_throw())
            .cartesian_product(dirac_throw())
            .map(|((t1,t2),t3)| t1+t2+t3).collect();

        for roll in rolls {
            let mut new_universe = universe.clone();
            let player = if p1_turn {&mut new_universe.player1} else {&mut new_universe.player2};
            play_turn_det(player, roll);
            if player.1 >= 21 {
                insert_or_increment(ended_games, p1_turn, *count);
            }else {
                insert_or_increment(&mut new_universes, new_universe, *count);
            }
        }
    }
    new_universes
}

/// Insert `val` into the hashmap, or increment existing entry with `val`
fn insert_or_increment<K: Eq + Hash>(hashmap: &mut HashMap<K, u64>, key: K, val: u64) {
    let n = hashmap.get_mut(&key);
    if n.is_some() {
        let n = n.unwrap();
        *n = *n + val;
    }else {
        hashmap.insert(key, val);
    }
}

/// The possible outcomes of the Dirac Die
fn dirac_throw() -> Vec<u32> {
    vec![1,2,3]
}


/// Plays a deterministic turn where a player rolled the number `roll`
fn play_turn_det(player: &mut Player, roll: u32) {
    let pos = player.0;
    let new_pos = (pos + roll - 1) % 10 + 1;
    player.1 += new_pos;
    player.0 = new_pos;
}

// --- PARSING ---
fn parse(input: &str) -> (Player, Player) {
    let mut lines = input.lines();
    let l1 = lines.next().unwrap();
    let l2 = lines.next().unwrap();
    let p1 = u32::from_str_radix(&l1.trim_end().chars().rev().next().unwrap().to_string(), 10).unwrap();
    let p2 = u32::from_str_radix(&l2.trim_end().chars().rev().next().unwrap().to_string(), 10).unwrap();
    (Player(p1, 0), Player(p2, 0))
}

// --- TEST INPUTS ---
fn test_input() -> String {
    String::from("Player 1 starting position: 4
Player 2 starting position: 8")
}
