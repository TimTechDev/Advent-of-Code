#[derive(PartialEq)]
enum PlatformTile {
    StoneRound,
    StoneCube,
    Empty,
}

fn parse_line(input: &str) -> Vec<PlatformTile> {
    input.chars().map(|c| match c {
        '.' => PlatformTile::Empty,
        '#' => PlatformTile::StoneCube,
        'O' => PlatformTile::StoneRound,
        _ => panic!("Cant parse line because {} is not . or #", c)
    }).collect()
}

#[aoc_generator(day14)]
pub fn parser(input: &str) -> Vec<Vec<PlatformTile>> {
    input.lines().map(parse_line).collect()
}

/* Solvers */

#[aoc(day14, part1)]
pub fn solver_part1(input: &[Vec<PlatformTile>]) -> u32 {
    let mut input = Vec::from_iter(input.iter().map(|x| Vec::from_iter(x.iter())));
    let l = input.len();
    for _ in 0..l {
        for i in 1..l {
            for j in 0..input[i].len() {
                if *input[i][j] != PlatformTile::StoneRound {
                    continue;
                }
                if *input[i - 1][j] != PlatformTile::Empty {
                    continue;
                }
                input[i - 1][j] = &PlatformTile::StoneRound;
                input[i][j] = &PlatformTile::Empty;
            }
        }
    }
    return input.iter().enumerate().map(|(i, x)| (l-i) as u32 * x.iter().filter(|y| *y == &&PlatformTile::StoneRound).count() as u32).sum::<u32>();
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "O....#....\nO.OO#....#\n.....##...\nOO.#O....O\n.O.....O#.\nO.#..O.#.#\n..O..#O..O\n.......O..\n#....###..\n#OO..#....";

    #[test]
    fn test_solver_part1() {
        assert_eq!(136, solver_part1(&parser(EXAMPLE_1)));
    }
}

