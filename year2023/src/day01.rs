#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> u32 {
    return input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .fold(0, |acc, n| (if acc == 0 { n } else { acc / 10 }) * 10 + n)
        })
        .sum();
}

#[aoc(day1, part2)]
fn solve_part2(input: &str) -> usize {
    return input
        .lines()
        .map(|line| {
            ("----".to_owned() + line)
                .chars()
                .collect::<Vec<char>>()
                .windows(5)
                .filter_map(|c| match c {
                    d if d[4] == '0' => None,
                    d if d[4].is_ascii_digit() => d[4].to_digit(10),
                    d if d[2..5] == ['o', 'n', 'e'] => Some(1),
                    d if d[2..5] == ['t', 'w', 'o'] => Some(2),
                    d if d[2..5] == ['s', 'i', 'x'] => Some(6),
                    d if d[1..5] == ['f', 'o', 'u', 'r'] => Some(4),
                    d if d[1..5] == ['f', 'i', 'v', 'e'] => Some(5),
                    d if d[1..5] == ['n', 'i', 'n', 'e'] => Some(9),
                    d if d == ['t', 'h', 'r', 'e', 'e'] => Some(3),
                    d if d == ['s', 'e', 'v', 'e', 'n'] => Some(7),
                    d if d == ['e', 'i', 'g', 'h', 't'] => Some(8),
                    _ => None,
                })
                .fold(0, |acc, n| (if acc == 0 { n } else { acc / 10 }) * 10 + n)
        })
        .map(|x| x as usize)
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
    const EXAMPLE_2: &str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";

    #[test]
    fn test_part1() {
        assert_eq!(142, solve_part1(EXAMPLE_1));
    }

    #[test]
    fn test_part2() {
        assert_eq!(281, solve_part2(EXAMPLE_2));
    }
}
