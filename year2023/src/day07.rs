use std::fmt::Debug;

fn type_helper(first: u8, second: u8) -> u8 {
    return match first {
        5 => 6,
        4 => 5,
        3 => (if second == 2 { 4 } else { 3 }),
        2 => (if second == 2 { 2 } else { 1 }),
        1 => 0,
        _ => unreachable!(),
    };
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub(crate) struct Hand<T> {
    pub(crate) cards: [T; 5],
    pub(crate) bid: i32,
}

mod part1 {
    use super::Hand;
    use core::hash::Hash;
    use std::{cmp::Ordering, collections::HashMap};

    #[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub(crate) enum Card {
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Jack,
        Queen,
        King,
        Ace,
    }

    impl From<char> for Card {
        fn from(value: char) -> Self {
            return match value {
                'A' => Self::Ace,
                'K' => Self::King,
                'Q' => Self::Queen,
                'J' => Self::Jack,
                'T' => Self::Ten,
                '9' => Self::Nine,
                '8' => Self::Eight,
                '7' => Self::Seven,
                '6' => Self::Six,
                '5' => Self::Five,
                '4' => Self::Four,
                '3' => Self::Three,
                '2' => Self::Two,
                _ => unreachable!(),
            };
        }
    }

    fn hand_type(hand: &Hand<Card>) -> u8 {
        let mut map: HashMap<Card, u8> = HashMap::new();
        for card in &hand.cards {
            map.insert(card.clone(), 1 + *map.get(card).unwrap_or(&0));
        }
        let mut amounts: Vec<&u8> = map.values().collect();
        amounts.sort();
        amounts.reverse();

        return super::type_helper(*amounts[0], **amounts.get(1).unwrap_or(&&0_u8));
    }

    impl Ord for Hand<Card> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            let type_order = hand_type(self).cmp(&hand_type(other));
            if type_order != Ordering::Equal {
                return type_order;
            }

            for i in 0..5 {
                let card_order = self.cards[i].cmp(&other.cards[i]);
                if card_order != Ordering::Equal {
                    return card_order;
                }
            }
            return Ordering::Equal;
        }
    }
}

mod part2 {
    use super::Hand;
    use core::hash::Hash;
    use std::{cmp::Ordering, collections::HashMap};

    #[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub(crate) enum Card {
        Jack,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Queen,
        King,
        Ace,
    }

    impl From<char> for Card {
        fn from(value: char) -> Self {
            return match value {
                'A' => Self::Ace,
                'K' => Self::King,
                'Q' => Self::Queen,
                'J' => Self::Jack,
                'T' => Self::Ten,
                '9' => Self::Nine,
                '8' => Self::Eight,
                '7' => Self::Seven,
                '6' => Self::Six,
                '5' => Self::Five,
                '4' => Self::Four,
                '3' => Self::Three,
                '2' => Self::Two,
                _ => unreachable!(),
            };
        }
    }

    fn hand_type(hand: &Hand<Card>) -> u8 {
        let mut map: HashMap<Card, u8> = HashMap::new();
        for card in &hand.cards {
            map.insert(card.clone(), 1 + *map.get(card).unwrap_or(&0));
        }

        let jacks = *map.get(&Card::Jack).unwrap_or(&0);
        map.remove(&Card::Jack);

        let mut amounts: Vec<&u8> = map.values().collect();
        amounts.sort();
        amounts.reverse();

        if jacks == 5 {
            return 6;
        }

        return super::type_helper(*amounts[0] + jacks, **amounts.get(1).unwrap_or(&&0_u8));
    }

    impl Ord for Hand<Card> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            let type_order = hand_type(self).cmp(&hand_type(other));
            if type_order != Ordering::Equal {
                return type_order;
            }

            for i in 0..5 {
                let card_order = self.cards[i].cmp(&other.cards[i]);
                if card_order != Ordering::Equal {
                    return card_order;
                }
            }
            return Ordering::Equal;
        }
    }
}

fn parser<T>(input: &str) -> Vec<Hand<T>>
where
    T: Debug + From<char>,
{
    return input
        .lines()
        .map(|line| Hand {
            cards: line
                .split_once(' ')
                .unwrap()
                .0
                .chars()
                .map(|c| T::from(c))
                .collect::<Vec<T>>()
                .try_into()
                .unwrap(),
            bid: line.split_once(' ').unwrap().1.parse().unwrap(),
        })
        .collect();
}


#[aoc_generator(day7, part1)]
fn parser_part1(input: &str) -> Vec<Hand<part1::Card>> {
    return parser(input);
}

#[aoc_generator(day7, part2)]
fn parser_part2(input: &str) -> Vec<Hand<part2::Card>> {
    return parser(input);
}

#[aoc(day7, part1)]
fn solver_part1(hands: &[Hand<part1::Card>]) -> i32 {
    let mut hands: Vec<Hand<part1::Card>> = hands.to_vec();
    hands.sort_by(Hand::cmp);
    return hands
        .iter()
        .enumerate()
        .map(|(n, hand)| ((n as i32) + 1) * hand.bid)
        .sum();
}

#[aoc(day7, part2)]
fn solver_part2(hands: &[Hand<part2::Card>]) -> i32 {
    let mut hands: Vec<Hand<part2::Card>> = hands.to_vec();
    hands.sort_by(Hand::cmp);
    return hands
        .iter()
        .enumerate()
        .map(|(n, hand)| ((n as i32) + 1) * hand.bid)
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";

    #[test]
    fn test_parser_part1() {
        let result = parser_part1(EXAMPLE_1);
        println!("{:?}", result);
    }

    #[test]
    fn test_solver_part1() {
        assert_eq!(6440, solver_part1(&parser_part1(EXAMPLE_1)));
    }

    #[test]
    fn test_solver_part2() {
        assert_eq!(5905, solver_part2(&parser_part2(EXAMPLE_1)));
    }
}
