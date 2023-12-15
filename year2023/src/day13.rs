fn parse_line(input: &str) -> Vec<u32> {
    input.chars().map(|c| match c {
        '.' => 0_u32,
        '#' => 1_u32,
        _ => panic!("Cant parse line because {} is not . or #", c)
    }).collect()
}

fn parse_block(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut rows:Vec<u32> = Vec::new();
    let mut cols:Vec<u32> = Vec::new();
    for line in input.lines().map(parse_line) {
        rows.push(line.iter().fold(0, |acc, &x| (acc << 1) + (x)));
        line.iter().enumerate().for_each(|(i, &x)| match cols.get_mut(i) {
            Some(y) => *y = (*y << 1) + (x),
            None => cols.push(x)
        })
    }
    return (rows, cols);
}

#[aoc_generator(day13)]
pub fn parser(input: &str) -> Vec<(Vec<u32>, Vec<u32>)> {
    input.split("\n\n").map(parse_block).collect()
}

/* Solvers */
fn solve_pattern_reflection(arr: &Vec<u32>) -> u32 {
    for (i, (a, b)) in arr.iter().zip(arr.iter().skip(1)).enumerate() {
        if a != b {
            continue;
        }
        if arr.iter().skip(i + 1).zip(arr.iter().take(i + 1).rev()).all(|(a, b)| a == b) {
            return (i as u32) + 1;
        }
    }
    return 0;
}

fn solve_pattern_smudge(arr: &Vec<u32>) -> u32 {
    let mut sums = vec![0; 2 * arr.len() - 1];
    arr.iter().enumerate().for_each(|(i, a)| arr.iter().enumerate().skip(i).for_each(|(j, b)| sums[i + j] = sums[i + j] + (a^b).count_ones()));
    let sums = sums.iter().enumerate().filter(|(i, _)| i % 2 == 1).map(|(i, x)| (((i as u32)+1)/2, *x)).collect::<Vec<(u32, u32)>>();
    for (i, sum) in sums {
        if sum == 1 {
            return i;
        }
    }
    return 0;
}

#[aoc(day13, part1)]
pub fn solver_part1(input: &[(Vec<u32>, Vec<u32>)]) -> u32 {
    return input.iter().map(|(rows, cols)| 100 * solve_pattern_reflection(rows) + solve_pattern_reflection(cols)).sum()
}


#[aoc(day13, part2)]
pub fn solver_part2(input: &[(Vec<u32>, Vec<u32>)]) -> u32 {
    return input.iter().map(|(rows, cols)| 100 * solve_pattern_smudge(rows) + solve_pattern_smudge(cols)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_BLOCK_1: &str = "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.";

    #[test]
    fn test_solver_part1() {
        assert_eq!(5, solver_part1(&vec![parse_block(EXAMPLE_BLOCK_1)]));
    }

    #[test]
    fn test_solver_part2() {
        assert_eq!(300, solver_part2(&vec![parse_block(EXAMPLE_BLOCK_1)]));
    }
}
