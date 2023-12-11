type Int = i64;

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<Vec<Int>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(str::parse)
                .map(Result::unwrap)
                .collect::<Vec<Int>>()
        })
        .collect()
}

fn extrapolate_forwards(v: &[Int]) -> Int {
    let mut last: Vec<Int> = Vec::new();
    let mut temp = v.to_owned();
    while temp.iter().any(|&x| x != 0) {
        last.push(*temp.last().unwrap());
        temp = temp.windows(2).map(|a| a[1] - a[0]).collect::<Vec<Int>>();
    }
    last.reverse();
    println!("{:?}", last);
    return last.iter().copied().reduce(|acc, x| acc + x).unwrap();
}

fn extrapolate_backwards(v: &[Int]) -> Int {
    let mut first: Vec<Int> = Vec::new();
    let mut temp = v.to_owned();
    while temp.iter().any(|&x| x != 0) {
        first.push(*temp.first().unwrap());
        temp = temp.windows(2).map(|a| a[1] - a[0]).collect::<Vec<Int>>();
    }
    first.reverse();
    println!("{:?}", first);
    return first.iter().copied().reduce(|acc, x| x - acc).unwrap();
}

#[aoc(day9, part1)]
pub fn solve_part1(data: &[Vec<Int>]) -> Int {
    return data.iter().map(|x| extrapolate_forwards(x)).sum();
}

#[aoc(day9, part2)]
fn solve_part2(data: &[Vec<Int>]) -> Int {
    return data.iter().map(|x| extrapolate_backwards(x)).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45";

    #[test]
    fn test_extrapolate_forwards() {
        assert_eq!(18, extrapolate_forwards(&vec![0, 3, 6, 9, 12, 15]));
        assert_eq!(28, extrapolate_forwards(&vec![1, 3, 6, 10, 15, 21]));
        assert_eq!(68, extrapolate_forwards(&vec![10, 13, 16, 21, 30, 45]));
    }

    #[test]
    fn test_extrapolate_backwards() {
        assert_eq!(-3, extrapolate_backwards(&vec![0, 3, 6, 9, 12, 15]));
        assert_eq!(0, extrapolate_backwards(&vec![1, 3, 6, 10, 15, 21]));
        assert_eq!(5, extrapolate_backwards(&vec![10, 13, 16, 21, 30, 45]));
    }

    #[test]
    fn test_part1() {
        assert_eq!(114, solve_part1(&parse(EXAMPLE_1)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(2, solve_part2(&parse(EXAMPLE_1)));
    }
}
