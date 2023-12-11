use std::collections::HashSet;

type ParsedInput = Vec<(usize, usize)>;

#[aoc_generator(day11)]
fn parser(input: &str) -> ParsedInput {
    let mut pairs: Vec<(usize, usize)> = Vec::new();
    for (ln, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                pairs.push((ln, col));
            }
        }
    }
    return pairs;
}

fn expanding_space(pairs: &ParsedInput) -> Vec<(usize, usize, usize, usize)> {
    let ln_set = pairs.iter().map(|&x| x.0).collect::<HashSet<usize>>();
    let col_set = pairs.iter().map(|&x| x.1).collect::<HashSet<usize>>();

    let max_ln = *ln_set.iter().max().unwrap_or(&0);
    let max_col = *col_set.iter().max().unwrap_or(&0);

    let mut expanding_lns: Vec<usize> = (0..max_ln).collect::<HashSet<usize>>()
        .difference(&ln_set)
        .copied()
        .collect();
    
    let mut expanding_cols: Vec<usize> = (0..max_col).collect::<HashSet<usize>>()
        .difference(&col_set)
        .copied()
        .collect();

    expanding_lns.sort_unstable();
    expanding_cols.sort_unstable();

    let mut result: Vec<(usize, usize, usize, usize)> = Vec::new();

    for pair in pairs {
        let ln_offset = expanding_lns.iter().filter(|&&x| x < pair.0).count();
        let col_offset = expanding_cols.iter().filter(|&&x| x < pair.1).count();
        result.push((pair.0, ln_offset, pair.1, col_offset));
    }
    return result;
}

fn distances(pairs: &[(usize, usize, usize, usize)], scalar: usize) -> usize {
    if scalar == 0 {
        panic!("At the disco!")
    }
    let scalar = scalar - 1;

    return pairs
        .iter()
        .enumerate()
        .flat_map(|(i, (a0, a1, a2, a3))| {
            pairs
                .iter()
                .skip(i + 1)
                .map(|(b0, b1, b2, b3)| ((*a0, *a1, *a2, *a3), (*b0, *b1, *b2, *b3)))
        })
        .map(|(a, b)| {
            a.0.abs_diff(b.0)
                + a.2.abs_diff(b.2)
                + a.1.abs_diff(b.1) * scalar
                + a.3.abs_diff(b.3) * scalar
        })
        .sum();
}

#[aoc(day11, part1)]
fn solver_part1(pairs: &ParsedInput) -> usize {
    return distances(&expanding_space(pairs), 2);
}

#[aoc(day11, part2)]
fn solver_part2(pairs: &ParsedInput) -> usize {
    return distances(&expanding_space(pairs), 1_000_000);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....";

    #[test]
    fn test_part1() {
        assert_eq!(0, solver_part1(&parser("")));
        assert_eq!(374, solver_part1(&parser(EXAMPLE_1)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, solver_part2(&parser("")));
        assert_eq!(82000210, solver_part2(&parser(&EXAMPLE_1)));
    }
}
