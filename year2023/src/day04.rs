#[derive(Debug)]
struct Card {
    winning: Vec<usize>,
    drawn: Vec<usize>,
}

fn parse_line(input: &str) -> Card {
    let (w, d) = input.split_once(':').unwrap().1.split_once('|').unwrap();

    return Card {
        winning: w
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect(),
        drawn: d
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect(),
    };
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<Card> {
    return input.lines().map(parse_line).collect();
}

fn part1_score(count: usize) -> usize {
    if count == 0 {
        return 0;
    }

    return 1 << (count - 1);
}

#[aoc(day4, part1)]
fn part1(cards: &[Card]) -> usize {
    return cards
        .iter()
        .map(|card| {
            part1_score(
                card.drawn
                    .iter()
                    .filter(|d| card.winning.contains(*d))
                    .count(),
            )
        })
        .sum();
}

#[aoc(day4, part2)]
fn part2(cards: &[Card]) -> usize {
    let mut amounts: Vec<usize> = cards.iter().map(|_| 1_usize).collect();
    for (position, card) in cards.iter().enumerate() {
        let count = card
            .drawn
            .iter()
            .filter(|d| card.winning.contains(*d))
            .count();
        let amount = *amounts.get(position).unwrap_or(&0);

        // println!("card: {} amount: {} count: {}", position + 1, amount, count);

        if count == 0 {
            continue;
        }

        for item in amounts.iter_mut().take(position + count + 1).skip(position + 1) {
            *item += amount;
        }
        // println!("amounts: {:?}", amounts);
    }

    return amounts.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_parser() {
        let result = parse(EXAMPLE_1);
        println!("{:?}", result);
        for card in result {
            assert_eq!(5, card.winning.len());
            assert_eq!(8, card.drawn.len());
        }
    }

    #[test]
    fn test_part1() {
        assert_eq!(13, part1(&parse(EXAMPLE_1)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(30, part2(&parse(EXAMPLE_1)));
    }
}
