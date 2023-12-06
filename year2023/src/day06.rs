type Int = i64;

#[derive(Debug)]

struct Race {
    time: Int,
    distance: Int,
}

fn parser_part1_line(line: &str) -> impl Iterator<Item = Int> + '_ {
    return line
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse::<Int>())
        .map(Result::unwrap);
}

#[aoc_generator(day6, part1)]
fn parser_part1(input: &str) -> Vec<Race> {
    let (times, distances) = input.split_once("\n").unwrap();
    return parser_part1_line(times)
        .zip(parser_part1_line(distances))
        .map(|(time, distance)| Race { time, distance })
        .collect();
}

fn parser_part2_line(line: &str) -> Int {
    line.chars()
        .filter(|c| c.is_digit(10))
        .fold(0, |acc, c| (acc * 10) + c.to_digit(10).unwrap() as Int)
}

#[aoc_generator(day6, part2)]
fn parser_part2(input: &str) -> Race {
    let (t, d) = input.split_once("\n").unwrap();
    return Race {
        time: parser_part2_line(t),
        distance: parser_part2_line(d),
    };
}

fn dist(time: Int, hold: Int) -> Int {
    (time - hold) * hold
}

#[aoc(day6, part1)]
fn solver_part1(data: &[Race]) -> Int {
    return data
        .iter()
        .map(|race| {
            (1..race.time)
                .filter(|&hold| dist(race.time, hold) > race.distance)
                .count()
        })
        .reduce(|a, b| a * b)
        .unwrap()
        .try_into()
        .unwrap();
}

#[aoc(day6, part2)]
fn solver_part2(race: &Race) -> Int {
    let mut first: Option<Int> = Option::None;
    let mut last: Option<Int>  = Option::None;
    for i in 1..race.time {
        let b = dist(race.time, i) > race.distance;
        if b {
            first = Some(i);
            break;
        }
    }
    for i in (1..race.time).rev() {
        let b = dist(race.time, i) > race.distance;
        if b {
            last = Some(i);
            break;
        }
    }

    return last.unwrap() - first.unwrap() + 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "Time:      7  15   30\nDistance:  9  40  200";

    #[test]
    fn test_parser_part1() {
        let result = parser_part1(EXAMPLE_1);
        println!("{:?}", result);
    }

    #[test]
    fn test_parser_part2() {
        let result = parser_part2(EXAMPLE_1);
        println!("{:?}", result);
    }

    #[test]
    fn test_solver_part1() {
        assert_eq!(288, solver_part1(&parser_part1(EXAMPLE_1)));
    }

    #[test]
    fn test_solver_part2() {
        assert_eq!(71503, solver_part2(&parser_part2(EXAMPLE_1)));
    }
}
