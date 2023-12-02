struct Draw {
    red: usize,
    green: usize,
    blue: usize,
}

pub struct Game {
    id: usize,
    draws: Vec<Draw>,
}

fn parse_cubes(input: &str) -> Draw {
    let mut red: usize = 0;
    let mut green: usize = 0;
    let mut blue: usize = 0;

    input.split(",").for_each(|s| {
        if s.contains("red") {
            red = s.strip_suffix("red").unwrap().trim().parse().unwrap_or(0)
        }
        if s.contains("green") {
            green = s.strip_suffix("green").unwrap().trim().parse().unwrap_or(0)
        }
        if s.contains("blue") {
            blue = s.strip_suffix("blue").unwrap().trim().parse().unwrap_or(0)
        }
    });
    return Draw { red, green, blue };
}

fn parse_line(input: &str) -> Game {
    let (a, b) = input.split_once(":").unwrap();
    return Game {
        id: a.strip_prefix("Game ").unwrap().parse::<usize>().unwrap(),
        draws: b.split(";").map(parse_cubes).collect::<Vec<Draw>>(),
    };
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<self::Game> {
    input.lines().map(parse_line).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(data: &[Game]) -> usize {
    return data
        .iter()
        .filter(|game| {
            game.draws
                .iter()
                .all(|draw| draw.red <= 12 && draw.green <= 13 && draw.blue <= 14)
        })
        .map(|game| game.id)
        .sum();
}

#[aoc(day2, part2)]
fn solve_part2(data: &[Game]) -> usize {
    return data
        .iter()
        .map(|game| {
            let red = game.draws.iter().map(|draw| draw.red).max().unwrap();
            let green = game.draws.iter().map(|draw| draw.green).max().unwrap();
            let blue = game.draws.iter().map(|draw| draw.blue).max().unwrap();
            return red * green * blue;
        })
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part1() {
        assert_eq!(8, solve_part1(&parse(EXAMPLE_1)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(2286, solve_part2(&parse(EXAMPLE_1)));
    }
}
