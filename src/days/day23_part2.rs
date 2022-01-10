use std::fmt;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::BinaryHeap;

/*
 * Day 23: Amphipod
 *
 * A bunch of amphipods are in a room. What is the least amount of energy required to organize them?
 *
 * Implementation: Copy and paste from part1, with some minor adjustments to allow for bigger rooms.
 */

struct DistanceMap(HashMap<State, u32>);
impl DistanceMap {
    fn new() -> Self {
        DistanceMap(HashMap::new())
    }
    fn update(&mut self, node: &State, distance: u32) {
        let entry = self.0.get_mut(&node);
        if entry.is_some() {
            let n = entry.unwrap();
            *n = *n + distance;
        }else {
            self.0.insert(node.clone(), distance);
        }
    }
    fn get(&mut self, node: &State) -> u32 {
        *self.0.get(node).unwrap_or(&u32::MAX)
    }
}

#[derive(Debug,Clone,Eq,PartialEq,Hash)]
struct Node{
    state: State,
    g: u32,
    h: u32,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
// This way we can turn BinaryHeap into a min-heap
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        let val1 = self.g + self.h;
        let val2 = other.g + other.h;
        val1.cmp(&val2).reverse()
    }
}


pub fn part2(input: &str) -> String {
    let start_state = parse(&input);
    // A* algorithm to find shortest path to goal state
    let goal = State::goal_state();
    let start_node = Node{state: start_state.clone(), g: 0, h: State::heuristic(&start_state)};

    let mut dist = DistanceMap::new();
    dist.update(&start_node.state, 0);

    let mut queue = BinaryHeap::new();
    queue.push(start_node);
    //let mut prev: HashMap<State, State> = HashMap::new(); // bookkeeping for retracing the path

    let mut result = None; 
    while let Some(n) = queue.pop() {
        if n.state == goal { result = Some((n.state, n.g)); break;}
        if n.g > dist.get(&n.state) {continue;} // there is a shorter way to reach the node
        
        let ns = n.state.neighbours();
        for (state, cost) in ns {
            let h = State::heuristic(&state);
            let node = Node{state,g: cost+n.g,h};
            if node.g < dist.get(&node.state) {
                //prev.insert(node.0.clone(), n.0.clone());
                dist.update(&node.state, node.g);
                queue.push(node);
            }
        }
    }
    // assume it worked
    let (_, energy) = result.unwrap();
    //// print path
    //let mut path = Vec::new();
    //path.push(state.clone());
    //let mut cur_state = state;
    //let prev_mut = &mut prev;
    //while let Some(s) = prev_mut.remove(&cur_state) {
    //    path.push(s.clone());
    //    cur_state = s;
    //}
    //println!("PATH");
    //for s in path.into_iter().rev() {
    //    println!("{}\n", s);
    //}

    format!("{}", energy)
}

#[derive(Debug,Hash,Eq,PartialEq,Copy,Clone)]
pub enum M {
    Empty,
    A,
    B,
    C,
    D
}

impl M {
    fn destination_rooms(&self) -> Vec<usize> {
        match self {
            M::Empty => {vec![]},
            M::A => {vec![11,12,13,14]},
            M::B => {vec![15,16,17,18]},
            M::C => {vec![19,20,21,22]},
            M::D => {vec![23,24,25,26]},
        }
    }
    fn step_cost(&self) -> u32 {
        match self {
            M::Empty => {panic!("Empty does not have a step cost")},
            M::A => {1},
            M::B => {10},
            M::C => {100},
            M::D => {1000},
        }
    }
}

#[derive(Debug,Hash,Eq,PartialEq,Clone)]
struct State {
    /*
     * Burrow encodes the possible states:
     *  0..11 is the hallway
     *  11..15 room 1 (Designated for A)
     *  15..19 room 2 (Designated for B)
     *  19..23 room 3 (Designated for C)
     *  23..27 room 4 (Designated for D)
     */
    burrow: Vec<M>
}

impl State {
    /// Returns a new state, in which the burrow is totally empty
    pub fn new() -> Self {
        State {
            burrow: vec![M::Empty; 27]
        }
    }

    /// Returns the goal state in which the amphipods are organized.
    pub fn goal_state() -> Self {
        let mut s = Self::new();
        let burrow = &mut s.burrow;
        for i in 0..4 {
            burrow[11+i] = M::A;
            burrow[15+i] = M::B;
            burrow[19+i] = M::C;
            burrow[23+i] = M::D;
        }
        s
    }

    /// Heuristic function for A* pathfinding
    /// Takes the manhattan distance from an amphipod to an unoccupied room.
    pub fn heuristic(state: &State) -> u32 {
        let types = vec![M::A,M::B,M::C,M::D];        
        let rooms = types.iter().map(|a| a.destination_rooms()).flatten();
        let mut misplaced_amphipods: Vec<_> = state.burrow.iter().enumerate().filter(|(i,&a)| a != M::Empty && !a.destination_rooms().contains(i)).collect();

        let mut minimal_energy_cost = 0;

        for room_index in rooms {
            let a = state.burrow[room_index];
            if !a.destination_rooms().contains(&room_index) {
                let r = ((room_index + 1) / 4) - 3;
                let desired_a = types[r];
                'inner: for i in 0..misplaced_amphipods.len() {
                    if *misplaced_amphipods[i].1 == desired_a {  // Found an amphipod suitable for the room
                        let (ii, a) = misplaced_amphipods.remove(i);
                        minimal_energy_cost += State::dist(room_index, ii) * a.step_cost();
                        break 'inner;
                    }
                }
            }
        }
        minimal_energy_cost
    }

    /// Distance from one location to another (ignoring any amphipods)
    fn dist(i1: usize, i2: usize) -> u32 {
        if i1 < 11 && i2 < 11 { // hallway to hallway
            return i32::abs(i2 as i32 -i1 as i32) as u32;
        } else if i1 < 11 || i2 < 11 { // hallway to room
            let (room, hallway) = if i1 >= 11 {(i1,i2)} else {(i2,i1)};
            let r = ((room + 1) / 4) - 3;
            let d = (room + 1) % 4;
            let hallway_diff = i32::abs(hallway as i32 - (r*2+2) as i32) as u32;
            return hallway_diff + d as u32 + 1;
        } else { // room to room
            let (r1, d1) = (((i1+1)/4) - 3, (i1+1) % 4);
            let (r2, d2) = (((i2+1)/4) - 3, (i2+1) % 4);
            if r1 == r2 {return i32::abs(d1 as i32 - d2 as i32) as u32;} // same room
            let (h1, h2) = ((r1*2+2), (r2*2+2)); // different rooms
            let hallway_diff = i32::abs(h1 as i32 - h2 as i32) as u32;
            return hallway_diff + d1 as u32 + d2 as u32 + 2;
        }
    }


    /// Returns neighbouring states and the costs to reach them.
    pub fn neighbours(&self) -> Vec<(Self, u32)> {
        let mut neighbours = Vec::new();
        for i in 0..self.burrow.len() {
            if self.burrow[i] != M::Empty {
                let mut legal_moves = self.moves(i); 
                neighbours.append(&mut legal_moves);
            }
        }
        neighbours
    }

    /// Checks legal moves for amphipod at index
    /// Includes pruning of the state space
    fn moves(&self, index: usize) -> Vec<(Self, u32)> {
        let amphipod = &self.burrow[index];
        let reachable: Vec<(usize, u32)> = self.reachable(index);
        let destination_rooms = amphipod.destination_rooms();
        // Check trivial case
        if index >= 11 && amphipod.destination_rooms().contains(&index) { // we are in a destination room.
            // don't move if we are in the correct spot
            let r = ((index + 1) / 4) - 3;
            let d = (index+1) % 4; 
            let mut different_species = false; // check whether there are any different species in the room
            for d in d+1..4 {
                if self.burrow[11+r*4+d] != *amphipod && self.burrow[11+r*4+d] != M::Empty {different_species = true;}
            }
            if !different_species {
                // try to move down as far as possible
                let max_move = (d+1..4).into_iter().enumerate().map(|(i, d)| ((i+1) as u32, 11+r*4+d)).take_while(|&(_, l)| self.burrow[l] == M::Empty).max();
                if max_move.is_some() {
                    let (s, l) = max_move.unwrap();
                    let mut new_state = self.clone();
                    new_state.burrow[index] = M::Empty;
                    new_state.burrow[l] = *amphipod;
                    return vec![(new_state, s * amphipod.step_cost())];

                }else {return vec![];} // we are at destination
            }
        }
        if index >= 11 { // try to move into the hallway
            let doors = State::doors();
            return reachable.into_iter()
                .filter(|(l, _)| *l < 11)
                .filter(|(l, _)| !doors.contains(l))
                .map(|(l, s)|{
                    let mut new_state = self.clone();
                    new_state.burrow[index] = M::Empty;
                    new_state.burrow[l] = *amphipod;
                    (new_state, s * amphipod.step_cost())
                }).collect();

        }else { // we are in the hallway
            let max_move = reachable.into_iter()
                .filter(|(l, _)| *l >= 11) // we can only move into a room
                .filter(|(l, _)| destination_rooms.contains(l)) // we can only move into our destination room
                .filter(|(l, _)| { // the destination room must only have our type of species
                    let r = ((*l + 1) / 4) - 3;
                    let mut coherency = true;
                    for d in 0..4 {
                        coherency = coherency && (self.burrow[11+d+r*4] == *amphipod || self.burrow[11+d+r*4] == M::Empty)
                    }
                    return coherency;
                }).max();
            return max_move.map_or(vec![], |(l, s)| {
                    let mut new_state = self.clone();
                    new_state.burrow[index] = M::Empty;
                    new_state.burrow[l] = *amphipod;
                    vec![(new_state, s * amphipod.step_cost())]
                });
        }
    }

    /// Returns a list of spaces reachable, without encountering another amphipod (exluding index)
    /// Does not take into account all of the rules
    /// Returns: (Index of reachable point, cost to reach in steps)
    fn reachable(&self, index: usize) -> Vec<(usize, u32)> {
        let mut reachable: Vec<(usize, u32)> = Vec::new();

        let (mut hallway_spot, mut hallway_cost): (usize, u32) = (index, 0); // (index, cost to reach)
        let mut room_r: i32 = -1; // potentially keep track of which room we were in
        if index >= 11 { // we are in a room. Check what other spots we can reach from it.
            // See how far we can go up, and how far we can go down in the room.
            let mut rooms = Vec::new();
            let depth = (index + 1) % 4; // 0 for front room, 3 for back room.
            let r = ((index + 1) / 4) - 3; // 0 for left-most room, 3 for right-most room. 
            room_r = r as i32;
            let mut up = (0..depth).into_iter().rev().enumerate()
                .map(|(c,d)| (11+4*r+d,c as u32+1)) // # of steps and location
                .take_while(|(l,_)| self.burrow[*l] == M::Empty);

            let mut down = (depth+1..4).into_iter().enumerate()
                .map(|(c,d)| (11+4*r+d,c as u32 + 1)) // # of steps and location
                .take_while(|(l,_)| self.burrow[*l] == M::Empty);

            rooms.extend(&mut down);
            rooms.extend(&mut up);
            // Check whether door is reachable
            let front_room = rooms.iter().find(|&(l,_)| *l == (11+r*4));
            if front_room.is_some(){ // we can reach the hallway
                let (_,s) = front_room.unwrap();
                hallway_spot = r*2+2;
                hallway_cost = (*s + 1) as u32;
                reachable.append(&mut rooms);
                reachable.push((hallway_spot, hallway_cost));
            }else if index == (11+r*4){
                hallway_spot = r*2+2;
                hallway_cost = 1;
                reachable.append(&mut rooms);
                reachable.push((hallway_spot, hallway_cost));
            }else { // we cannot reach the hallway
                return rooms;
            }
        }

        // check what we can reach from the hallway spot including its cost
        let mut reachable_hallway_spots: Vec<(usize, u32)> = Vec::new();
        reachable_hallway_spots.extend((0..hallway_spot).rev().enumerate().take_while(|&(_, l)| self.burrow[l] == M::Empty).map(|(i, l)| (l, i as u32 + hallway_cost+ 1)));
        reachable_hallway_spots.extend((hallway_spot+1..11).enumerate().take_while(|&(_, l)| self.burrow[l] == M::Empty).map(|(i, l)| (l, i as u32 + hallway_cost+ 1,)));
        // check for every spot whether we can reach a room and its subrooms (for the rooms we were not in initially)
        for (i, c) in reachable_hallway_spots.iter().chain(vec![(hallway_spot, hallway_cost)].iter()) {
            if *i == 2 && room_r != 0 {
                reachable.extend((11..15).enumerate().take_while(|&(_, l)| self.burrow[l] == M::Empty).map(|(s, l)| (l, s as u32 + c + 1)));
            }else if *i == 4 && room_r != 1 {
                reachable.extend((15..19).enumerate().take_while(|&(_, l)| self.burrow[l] == M::Empty).map(|(s, l)| (l, s as u32 + c + 1)));
            }else if *i == 6 && room_r != 2 {
                reachable.extend((19..23).enumerate().take_while(|&(_, l)| self.burrow[l] == M::Empty).map(|(s, l)| (l, s as u32 + c + 1)));
            }else if *i == 8 && room_r != 3 {
                reachable.extend((23..27).enumerate().take_while(|&(_, l)| self.burrow[l] == M::Empty).map(|(s, l)| (l, s as u32 + c + 1)));
            }
        }
        reachable.append(&mut reachable_hallway_spots);
        reachable
    }

    fn doors() -> Vec<usize> { vec![2,4,6,8] }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "#############")?;
        write!(f, "#")?;
        for i in 0..11 {
            match self.burrow[i] {
                M::Empty => {write!(f, ".")?;},
                M::A => {write!(f, "A")?;},
                M::B => {write!(f, "B")?;},
                M::C => {write!(f, "C")?;},
                M::D => {write!(f, "D")?;},
            }
        }
        writeln!(f, "#")?;
        write!(f, "###")?;
        for i in 0..4 {
            match self.burrow[11+i*4] {
                M::Empty => {write!(f, ".")?;},
                M::A => {write!(f, "A")?;},
                M::B => {write!(f, "B")?;},
                M::C => {write!(f, "C")?;},
                M::D => {write!(f, "D")?;},
            }
            write!(f, "#")?;
        }
        writeln!(f, "##")?;
        for r in 1..4 { 
            write!(f, "  #")?;
            for i in 0..4 {
                match self.burrow[11+r+i*4] {
                    M::Empty => {write!(f, ".")?;},
                    M::A => {write!(f, "A")?;},
                    M::B => {write!(f, "B")?;},
                    M::C => {write!(f, "C")?;},
                    M::D => {write!(f, "D")?;},
                }
                write!(f, "#")?;
            }
            writeln!(f, "  ")?;
        }
        write!(f, "  #########  ")
    }
}

// --- PARSING ---
fn parse(input: &str) -> State {
    let mut lines = input.lines().skip(2);
    let mut row1 = lines.next().unwrap().chars().skip(3).step_by(2);
    let mut state = State::new();
    for i in 0..4 {
        match row1.next().unwrap() {
            'A' => {state.burrow[11+i*4] = M::A},
            'B' => {state.burrow[11+i*4] = M::B},
            'C' => {state.burrow[11+i*4] = M::C},
            'D' => {state.burrow[11+i*4] = M::D},
            a => {panic!("Unrecognized Character: {}", a)},
        }
    }

    // Insert extra part between top and bottom row.
    let burrow = &mut state.burrow;
    burrow[12] = M::D;
    burrow[13] = M::D;
    burrow[16] = M::C;
    burrow[17] = M::B;
    burrow[20] = M::B;
    burrow[21] = M::A;
    burrow[24] = M::A;
    burrow[25] = M::C;

    let mut row2 = lines.next().unwrap().chars().skip(3).step_by(2);
    for i in 0..4 {
        match row2.next().unwrap() {
            'A' => {state.burrow[14+i*4] = M::A},
            'B' => {state.burrow[14+i*4] = M::B},
            'C' => {state.burrow[14+i*4] = M::C},
            'D' => {state.burrow[14+i*4] = M::D},
            a => {panic!("Unrecognized Character: {}", a)},
        }
    }
    return state;
}

// --- TEST INPUTS ---
fn test_input() -> String {
    String::from("#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########")
}

// --- TESTS ---
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_state() {
        let mut state1 = State::new();

        // Test Reachability
        let reachable = state1.reachable(0);
        println!("Testing Reachability from: {}", 0);
        println!("{}", state1);
        println!("{:?}", reachable);
        assert_eq!(reachable.len(), 26);

        let reachable = state1.reachable(12);
        println!("Testing Reachability from: {}", 12);
        println!("{}", state1);
        println!("{:?}", reachable);
        assert_eq!(reachable.len(), 26);

        state1.burrow[8] = M::A;
        let reachable = state1.reachable(12);
        println!("Testing Reachability from: {}", 12);
        println!("{}", state1);
        println!("{:?}", reachable);
        assert_eq!(reachable.len(), 19);
        state1.burrow[8] = M::Empty;

        state1.burrow[9] = M::A;
        let reachable = state1.reachable(10);
        println!("Testing Reachability from: {}", 10);
        println!("{}", state1);
        println!("{:?}", reachable);
        assert_eq!(reachable.len(), 0);
        state1.burrow[9] = M::Empty;

        state1.burrow[9] = M::A;
        let reachable = state1.reachable(11);
        println!("Testing Reachability from: {}", 11);
        println!("{}", state1);
        println!("{:?}", reachable);
        assert_eq!(reachable.len(), 24);
        state1.burrow[9] = M::Empty;

        state1.burrow[18] = M::A;
        state1.burrow[12] = M::B;
        state1.burrow[8] = M::C;
        state1.burrow[14] = M::D;
        let reachable = state1.reachable(15);
        println!("Testing Reachability from: {}", 15);
        println!("{}", state1);
        println!("{:?}", reachable);
        assert_eq!(reachable.len(), 15);

        let goal = State::goal_state();
        let mut state = State::new();
        for r in 0..4 {
            for d in 0..4 {
                if r == 0 {
                    state.burrow[11+r*4+d] = M::A;
                }else if r == 1 {
                    state.burrow[11+r*4+d] = M::B;
                }else if r == 2 {
                    state.burrow[11+r*4+d] = M::C;
                }else {
                    state.burrow[11+r*4+d] = M::D;
                }
            }
        }
        assert!(goal == state);
    }
}
