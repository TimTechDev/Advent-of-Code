#[derive(Debug)]
struct Number {
    line: usize,
    column: usize,
    length: usize,
    value: usize,
}

#[derive(Debug)]
struct Symbol {
    line: usize,
    column: usize,
    value: char,
}

#[derive(Debug)]
struct Schematic {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Schematic {
    let mut line: usize = 0;
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    for ln in input.lines() {
        let mut column: usize = 0;

        let mut buffer: usize = 0;
        let mut start: usize = 0;

        ln.chars().for_each(|c| {
            match c {
                '.' => {
                    if buffer != 0 {
                        numbers.push(Number {
                            line,
                            column: start,
                            length: column - start,
                            value: buffer,
                        });
                        buffer = 0;
                    }
                }
                x if x.is_digit(10) => {
                    if buffer == 0 {
                        start = column;
                        buffer = x.to_digit(10).unwrap() as usize;
                    } else {
                        buffer = (buffer * 10) + x.to_digit(10).unwrap() as usize;
                    }
                }
                _ => {
                    symbols.push(Symbol {
                        line,
                        column,
                        value: c,
                    });
                    if buffer != 0 {
                        numbers.push(Number {
                            line,
                            column: start,
                            length: column - start,
                            value: buffer,
                        });
                        buffer = 0;
                    }
                }
            };
            column += 1;
        });
        if buffer != 0 {
            numbers.push(Number {
                line,
                column: start,
                length: column - start,
                value: buffer,
            });
        }
        line += 1;
    }
    // println!("|n| = {:?}  |s| = {:?}", numbers.len(), symbols.len());
    return Schematic { numbers, symbols };
}

#[aoc(day3, part1)]
fn part1(schematic: &Schematic) -> usize {
    return schematic
        .numbers
        .iter()
        .filter(|n| {
            let end_ln = n.line + 2;
            let lines = n.line..=end_ln;
            let end_col = n.column + n.length + 1;
            let columns = n.column..=end_col;

            return schematic
                .symbols
                .iter()
                .any(|s| lines.contains(&(s.line + 1)) && columns.contains(&(s.column + 1)));
        })
        .map(|n| n.value)
        .sum();
}

#[aoc(day3, part2)]
fn part2(schematic: &Schematic) -> usize {
    let result: usize = schematic
        .symbols
        .iter()
        .filter(|s| s.value == '*')
        .map(|s| {
            let numbers: Vec<&Number> = schematic
                .numbers
                .iter()
                .filter(|n| {
                    let end_ln = n.line + 2;
                    let lines = n.line..=end_ln;
                    let end_col = n.column + n.length + 1;
                    let columns = n.column..=end_col;
                    return lines.contains(&(s.line + 1)) && columns.contains(&(s.column + 1));
                })
                .collect();

            if numbers.len() != 2 {
                return 0 as usize;
            }

            return numbers[0].value * numbers[1].value; // TODO
        })
        .sum();

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";

    #[test]
    fn test_parser() {
        let result = parse(EXAMPLE_1);
        println!("{:?}", result);
    }

    #[test]
    fn test_part1() {
        assert_eq!(4361, part1(&parse(EXAMPLE_1)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(467835, part2(&parse(EXAMPLE_1)));
    }
}