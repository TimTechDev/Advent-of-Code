pub type Int = i64;

mod partial_fn {
    use std::ops::Range;

    use super::Int;

    #[derive(Debug, PartialEq, Clone)]
    pub struct FunctionPart {
        pub src: Range<Int>,
        pub dest: Range<Int>,
    }

    impl FunctionPart {
        pub fn new(src0: Int, dest0: Int, len: Int) -> Self {
            return Self {
                src: src0..(src0 + len),
                dest: dest0..(dest0 + len),
            };
        }
    }

    #[derive(Debug, Clone)]
    pub struct PartialFunction {
        pub parts: Vec<FunctionPart>,
    }

    impl PartialFunction {
        pub fn from(parts: Vec<FunctionPart>) -> Self {
            return Self { parts };
        }
        pub fn apply(&self, input: Int) -> Int {
            return match self.parts.iter().filter(|x| x.src.contains(&input)).last() {
                Some(map) => input - map.src.start + map.dest.start,
                None => input,
            };
        }
    }
}

use std::ops::Range;

use partial_fn::{FunctionPart, PartialFunction};

#[derive(Debug)]
struct Almanac {
    seed_data: Vec<Int>,
    mappings: Vec<PartialFunction>,
}

fn parse_block(input: &str) -> PartialFunction {
    return PartialFunction::from(
        input
            .lines()
            .skip(1)
            .map(|line| {
                let n: Vec<Int> = line
                    .trim()
                    .split_ascii_whitespace()
                    .map(|s| s.parse::<Int>())
                    .map(Result::unwrap)
                    .collect();
                return FunctionPart::new(n[1], n[0], n[2]);
            })
            .collect(),
    );
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Almanac {
    let seeds = input
        .split_once('\n')
        .unwrap()
        .0
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<Int>())
        .map(Result::unwrap)
        .collect();
    let mappings = input.split("\n\n").skip(1).map(parse_block).collect();
    return Almanac {
        seed_data: seeds,
        mappings,
    };
}

#[aoc(day5, part1)]
fn part1(almanac: &Almanac) -> Int {
    let result = almanac
        .seed_data
        .iter()
        .map(|seed| almanac.mappings.iter().fold(*seed, |l, b| b.apply(l)))
        .min();
    return result.unwrap();
}

#[cfg(feature = "bruteforce")]
#[aoc(day5, part2, bruteforce)]
fn part2(almanac: &Almanac) -> Int {
    let seed_ranges: Vec<Range<Int>> = almanac
        .seed_data
        .chunks(2)
        .map(|x| (x[0], x[1]))
        .map(|(start, len)| (start..(start + len)))
        .collect();

    let mut handles = vec![];

    for range in seed_ranges {
        let mappings = almanac.mappings.clone();
        let handle = std::thread::spawn(move || {
            let mut low: Option<Int> = None;
            for seed in range {
                let res = mappings.iter().fold(seed, |l, b| b.apply(l));
                if !low.is_some_and(|v| v < res) {
                    low = Some(res);
                }
            }
            return low.unwrap_or(Int::MAX);
        });
        handles.push(handle);
    }

    let mut results = vec![];

    for handle in handles {
        results.push(handle.join().unwrap());
    }

    return *results.iter().min().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) const EXAMPLE_1: &str = "seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15\n\nfertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4\n\nwater-to-light map:\n88 18 7\n18 25 70\n\nlight-to-temperature map:\n45 77 23\n81 45 19\n68 64 13\n\ntemperature-to-humidity map:\n0 69 1\n1 0 69\n\nhumidity-to-location map:\n60 56 37\n56 93 4";

    #[test]
    fn test_parser() {
        let result = parse(EXAMPLE_1);
        println!("{:?}", result);
    }

    #[test]
    fn test_part1() {
        assert_eq!(35, part1(&parse(EXAMPLE_1)));
    }

    #[cfg(feature = "bruteforce")]
    #[test]
    fn test_part2() {
        assert_eq!(46, part2(&parse(EXAMPLE_1)));
    }
}
