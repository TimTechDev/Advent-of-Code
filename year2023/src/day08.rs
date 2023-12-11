use std::{
    collections::HashMap,
    ops::{Div, Mul, Rem},
};

pub struct Network {
    inner: HashMap<u16, (u16, u16)>,
}

impl Network {
    fn with_capacity(capacity: usize) -> Self {
        return Self {
            inner: HashMap::with_capacity(capacity),
        };
    }

    fn str_to_u16(value: &str) -> u16 {
        if value.chars().count() != 3 {
            unreachable!();
        }

        return value
            .chars()
            .map(|c| {
                let v = u32::from(c) - u32::from('A');
                if !(0..26).contains(&v) {
                    unreachable!("{}({}) is not A-Z", c, v);
                }
                return v as u16;
            })
            .reduce(|acc, x| acc * 26 + x)
            .unwrap();
    }

    fn insert(&mut self, key: &str, value: (&str, &str)) -> Option<(u16, u16)> {
        return self.inner.insert(
            Self::str_to_u16(key),
            (Self::str_to_u16(value.0), Self::str_to_u16(value.1)),
        );
    }
}

#[derive(Debug)]
pub enum Instruction {
    Left,
    Right,
}

impl From<char> for Instruction {
    fn from(value: char) -> Self {
        return match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!(),
        };
    }
}

fn gcd<T: Copy + Eq + Ord + Rem<Output = T>>(zero: T, mut a: T, mut b: T) -> T {
    while b != zero {
        let t = b;
        b = a.rem(b);
        a = t;
    }
    return a;
}

fn lcm<T: Copy + Eq + Ord + Mul<Output = T> + Div<Output = T> + Rem<Output = T>>(
    zero: T,
    a: T,
    b: T,
) -> T {
    return a.mul(b.div(gcd(zero, a, b)));
}

#[aoc_generator(day8)]
pub fn parser(input: &str) -> (Vec<Instruction>, Network) {
    let parts = input.split_once("\n\n").unwrap();
    let instructions: Vec<Instruction> = parts.0.trim().chars().map(Instruction::from).collect();
    let mut network: Network = Network::with_capacity(parts.1.lines().count());
    for line in parts.1.lines() {
        let lparts = line.split_once('=').unwrap();
        let key = lparts.0.trim();
        let values = lparts.1.split_once(',').unwrap();
        let v1 = values.0.trim().strip_prefix('(').unwrap();
        let v2 = values.1.trim().strip_suffix(')').unwrap();
        network.insert(key, (v1, v2));
    }
    return (instructions, network);
}

#[aoc(day8, part1)]
pub fn solver_part1((instructions, network): &(Vec<Instruction>, Network)) -> i32 {
    let mut instructions = instructions.iter().cycle();
    let mut steps = 0;
    let mut current_key: u16 = Network::str_to_u16("AAA");

    while current_key != Network::str_to_u16("ZZZ") {
        let instruction = instructions.next().unwrap();
        let (left, right) = network.inner.get(&current_key).unwrap();
        current_key = match instruction {
            Instruction::Left => *left,
            Instruction::Right => *right,
        };
        steps += 1;
    }

    return steps;
}

#[aoc(day8, part2)]
pub fn solver_part2((instr, network): &(Vec<Instruction>, Network)) -> u64 {
    let mut instructions = instr.iter().cycle();
    let nodes: Vec<u16> = network
        .inner
        .keys()
        .filter(|&x| (x % 26) == 0)
        .copied()
        .collect();
    let mut cycles: Vec<u64> = vec![];

    println!("|nodes| = {}", nodes.len());

    for node in nodes {
        let mut current_key = node;
        let mut cycle = 0;

        while current_key % 26 != 25 {
            let instruction = instructions.next().unwrap();
            let (left, right) = network.inner.get(&current_key).unwrap();
            current_key = match instruction {
                Instruction::Left => *left,
                Instruction::Right => *right,
            };
            cycle += 1;
        }
        cycles.push(cycle);
    }

    println!("cycle lengths: {:?}", cycles);

    return cycles.iter().copied().reduce(|a, b| lcm(0, a, b)).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "RL\n\nAAA = (BBB, CCC)\nBBB = (DDD, EEE)\nCCC = (ZZZ, GGG)\nDDD = (DDD, DDD)\nEEE = (EEE, EEE)\nGGG = (GGG, GGG)\nZZZ = (ZZZ, ZZZ)";
    const EXAMPLE_2: &str = "LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)";
    const EXAMPLE_3: &str = "LR\n\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)";

    #[test]
    fn test_part1() {
        assert_eq!(2, solver_part1(&parser(EXAMPLE_1)));
        assert_eq!(6, solver_part1(&parser(EXAMPLE_2)));
    }

    #[test]
    fn test_lcm() {
        assert_eq!(805261, lcm(0_u64, 18727, 13201));
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            6,
            solver_part2(&parser(&EXAMPLE_3.replace("1", "F").replace("2", "H")))
        );
        assert_eq!(
            10921547990923,
            solver_part2(&parser(include_str!("../input/2023/day8.txt")))
        )
    }

    fn _u16_to_str(value: u16) -> String {
        let c1 = ((value / (26 * 26)) % 26) as u32 + u32::from('A');
        let c2 = ((value / 26) % 26) as u32 + u32::from('A');
        let c3 = (value % 26) as u32 + u32::from('A');
        return format!(
            "{}{}{}",
            char::from_u32(c1).unwrap(),
            char::from_u32(c2).unwrap(),
            char::from_u32(c3).unwrap()
        );
    }

    fn _print_diagramm() {
        let (_, network) = parser(include_str!("../input/2023/day8.txt"));
        let mut network: Vec<(u16, (u16, u16))> =
            network.inner.iter().map(|(&k, &v)| (k, v)).collect();
        network.sort_by_cached_key(|x| {
            (x.0 % 26) * (26 * 26) + ((x.0 / 26) % 26) * 26 + ((x.0 / (26 * 26)) % 26)
        });
        println!("stateDiagram-v2");
        for node in network {
            println!(
                "    {} --> {}",
                _u16_to_str(node.0),
                _u16_to_str((node.1).0)
            );
            println!(
                "    {} --> {}",
                _u16_to_str(node.0),
                _u16_to_str((node.1).1)
            );
        }
    }
}
