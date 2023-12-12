use std::fmt::{Debug, Write};

use crate::helpers::graph::{NodeIndex, UndirectedGraph};

type Grid = UndirectedGraph<Node, ()>;

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

use Direction::*;

#[derive(Clone, PartialEq)]
enum Node {
    Start,
    Pipe(bool, bool, bool, bool),
    None,
}

enum Edge {
    GridH,// ToDo 
    GridV,
    Pipe
}

impl From<char> for Node {
    fn from(value: char) -> Self {
        return match value {
            '|' => Self::Pipe(true, false, true, false),
            '-' => Self::Pipe(false, true, false, true),
            'L' => Self::Pipe(true, true, false, false),
            'J' => Self::Pipe(true, false, false, true),
            'F' => Self::Pipe(false, true, true, false),
            '7' => Self::Pipe(false, false, true, true),
            'S' => Self::Start,
            _ => Self::None,
        };
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Self::Pipe(true, false, true, false) => '│', // '┃'
            Self::Pipe(false, true, false, true) => '─', // '━'
            Self::Pipe(true, true, false, false) => '└', // '┗'
            Self::Pipe(true, false, false, true) => '┘', // '┛'
            Self::Pipe(false, true, true, false) => '┌', // '┏'
            Self::Pipe(false, false, true, true) => '┐', // '┓'
            Self::Pipe(_, _, _, _) => char::REPLACEMENT_CHARACTER,
            Self::Start => 'S',
            Self::None => '.',
        })
    }
}

impl Node {
    fn has_connection(&self, dir: Direction) -> bool {
        return match *self {
            Node::Start => true,
            Node::Pipe(n, e, s, w) => match dir {
                North => n,
                East => e,
                South => s,
                West => w,
            },
            Node::None => false,
        };
    }
}

fn add_connection(
    graph: &mut Grid,
    a: NodeIndex,
    dir_a: Direction,
    b: NodeIndex,
    dir_b: Direction,
) {
    let an = graph.get_node(a).unwrap();
    let bn = graph.get_node(b).unwrap();

    assert_ne!(a, b);

    if an.has_connection(dir_a) && bn.has_connection(dir_b) {
        graph.add_edge(b, a, ()).unwrap();
    }
}

#[aoc_generator(day10)]
fn parse(input: &str) -> (Grid, NodeIndex) {
    let mut graph = UndirectedGraph::new();

    let mut start: Option<NodeIndex> = None;

    let mut prev: Vec<NodeIndex> = Vec::new();
    let mut current: Vec<NodeIndex> = Vec::new();

    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            let data = Node::from(c);
            let node = graph.add_node(data.clone());
            current.push(node);

            if data == Node::Start {
                start = Some(node);
            }

            // connect left
            if i > 0 {
                let left = current.get(i.checked_sub(1).unwrap());
                add_connection(&mut graph, node, West, *left.unwrap(), East)
            }

            // conect top
            let top = prev.get(i);
            if top.is_some() {
                add_connection(&mut graph, node, North, *top.unwrap(), South)
            }
        }
        prev = current;
        current = Vec::new();
    }

    return (graph, start.unwrap());
}

#[aoc(day10, part1)]
fn solve_part1((grid, start): &(Grid, NodeIndex)) -> usize {
    let mut indices: Vec<NodeIndex> = Vec::new();
    let mut current = *start;

    loop {
        let neighbors: Vec<NodeIndex> = grid
            .neighbors(current)
            .unwrap()
            .filter(|n| !indices.contains(n))
            .collect();
        match neighbors.first() {
            Some(n) => current = *n,
            None => {
                break;
            }
        };

        indices.push(current);
    }
    return indices.len() / 2;
}

#[aoc(day10, part2)]
fn solve_part2((grid, start): &(Grid, NodeIndex)) -> usize {

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1_1: &str = ".....\n.S-7.\n.|.|.\n.L-J.\n.....";
    const EXAMPLE_1_2: &str = "..F7.\n.FJ|.\nSJ.L7\n|F--J\nLJ...";

    const EXAMPLE_2_1: &str = "...........\n.S-------7.\n.|F-----7|.\n.||.....||.\n.||.....||.\n.|L-7.F-J|.\n.|..|.|..|.\n.L--J.L--J.\n...........";
    const EXAMPLE_2_2: &str = "..........\n.S------7.\n.|F----7|.\n.||....||.\n.||....||.\n.|L-7F-J|.\n.|II||II|.\n.L--JL--J.\n..........";
    const EXAMPLE_2_3: &str = ".F----7F7F7F7F-7....\n.|F--7||||||||FJ....\n.||.FJ||||||||L7....\nFJL7L7LJLJ||LJ.L-7..\nL--J.L7...LJS7F-7L7.\n....F-J..F7FJ|L7L7L7\n....L7.F7||L7|.L7L7|\n.....|FJLJ|FJ|F7|.LJ\n....FJL-7.||.||||...\n....L---J.LJ.LJLJ...";
    const EXAMPLE_2_4: &str = "FF7FSF7F7F7F7F7F---7\nL|LJ||||||||||||F--J\nFL-7LJLJ||||||LJL-77\nF--JF--7||LJLJ7F7FJ-\nL---JF-JLJ.||-FJLJJ7\n|F|F-JF---7F7-L7L|7|\n|FFJF7L7F-JF7|JL---7\n7-L-JL7||F7|L7F-7F7|\nL.L7LFJ|||||FJL7||LJ\nL7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn test_part1() {
        assert_eq!(4, solve_part1(&parse(EXAMPLE_1_1)));
        assert_eq!(8, solve_part1(&parse(EXAMPLE_1_2)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(4, solve_part2(&parse(EXAMPLE_2_1)));
        assert_eq!(4, solve_part2(&parse(EXAMPLE_2_2)));
        assert_eq!(8, solve_part2(&parse(EXAMPLE_2_3)));
        assert_eq!(10, solve_part2(&parse(EXAMPLE_2_4)));
    }
}






