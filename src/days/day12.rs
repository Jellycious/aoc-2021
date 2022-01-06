use crate::AOCDay;

use std::collections::HashMap;
use std::collections::HashSet;

/*
 * Template for a implementing a day
 */

pub struct Day12();

impl AOCDay for Day12 {
    fn part1(&self, _input: &str) -> Option<String> { Some(part1(_input)) }
    fn part2(&self, _input: &str) -> Option<String> { Some(part2(_input)) }
    fn get_num(&self) -> u32 { 12 }
}

type Edge<'a> = (&'a str, &'a str);
type Graph<'a> = HashMap<&'a str, HashSet<&'a str>>;
type Path<'a> = Vec<&'a str>;

pub fn get() -> Day12 {Day12()}

fn part1(input: &str) -> String {
    let graph = graph(parser(&input));
    let paths = paths(&graph);
    String::from(format!("{}", paths))
}

fn part2(input: &str) -> String {
    let graph = graph(parser(&input));
    let paths = paths2(&graph);
    String::from(format!("{}", paths))
}

fn paths<'a>(graph: &'a Graph<'a>) -> u32 {
    let mut visited = HashSet::new();
    visited.insert("start");
    visit_node("start", graph, &visited)
}

fn paths2<'a>(graph: &'a Graph<'a>) -> u32 {
    let mut visited = HashSet::new();
    visited.insert("start");
    visit_node2("start", graph, &visited, false)
}

fn visit_node<'a>(node: &'a str, graph: &'a Graph, visited: &HashSet<&'a str>) -> u32 {
    if node == "end" { // reached end node
        return 1;
    }

    let neighbours = graph.get(node).unwrap();
    let mut paths = 0;

    for n in neighbours.iter() {
        if big(n) { // visit cave unconditionally
                let ps = visit_node(n, graph, visited);
                paths += ps;
        }else { // only visit cave if not visited already
            if !visited.contains(n) { // unvisited
                let mut visited = visited.clone();
                visited.insert(n);
                let ps = visit_node(n, graph, &visited);
                paths += ps;
            }
        }
    }
    paths
}

fn visit_node2<'a>(node: &'a str, graph: &'a Graph, visited: &HashSet<&'a str>, visited_node_twice: bool) -> u32 {
    if node == "end" { // reached end node
        return 1;
    }

    let neighbours = graph.get(node).unwrap();
    let mut paths = 0;

    for n in neighbours.iter() {
        if n == &"start" {continue;} // we do not visit start twice
        if big(n) { // visit cave unconditionally
                let ps = visit_node2(n, graph, visited, visited_node_twice);
                paths += ps;
        }else { // only visit cave if not visited already
            if visited_node_twice {
                // we only want to visit nodes, not visited already
                if !visited.contains(n) { // unvisited
                    let mut visited = visited.clone();
                    visited.insert(n);
                    let ps = visit_node2(n, graph, &visited, visited_node_twice);
                    paths += ps;
                }

            }else {
                // we do not care about visiting a node twice
                let mut visited = visited.clone();
                let visited_already = visited.contains(n);
                visited.insert(n);
                let ps = visit_node2(n, graph, &visited, visited_already);
                paths += ps;
            }
        }
    }
    paths
}

fn big(cave: &str) -> bool {
    cave.chars().all(|c| c.is_uppercase())
}

fn test_input1() -> String {
    String::from("start-A
start-b
A-c
A-b
b-d
A-end
b-end")
}

fn test_input2() -> String {
    String::from("dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc")
}

fn graph<'a>(edges: Vec<Edge<'a>>) -> Graph<'a> {
    let mut m: HashMap<&'a str, HashSet<&'a str>> = HashMap::new();
    for (n1, n2) in edges {
        if m.contains_key(n1) {
            let hs = m.get_mut(n1).unwrap();
            hs.insert(n2);
        }else {
            let mut hs = HashSet::new();
            hs.insert(n2);
            m.insert(n1, hs);
        }
        if m.contains_key(n2) {
            let hs = m.get_mut(n2).unwrap();
            hs.insert(n1);
        }else {
            let mut hs = HashSet::new();
            hs.insert(n1);
            m.insert(n2, hs);
        }
    }
    m
}

fn parser<'a>(input: &'a str) -> Vec<Edge<'a>> {
    input.lines().map(|l| {
        let mut ns = l.split("-");
        let n1 = ns.next().unwrap();
        let n2 = ns.next().unwrap();
        (n1, n2)
    }).collect()
}
