use std::{collections::HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    Eastt,
    South,
    Westt,
}
use Direction::*;

impl Direction {
    fn step(&self, other: (i32, i32)) -> Option<(i32, i32)> {
        return match self {
            North => Some((other.0.checked_sub(1)?, other.1)),
            Eastt => Some((other.0, other.1.checked_add(1)?)),
            South => Some((other.0.checked_add(1)?, other.1)),
            Westt => Some((other.0, other.1.checked_sub(1)?)),
        };
    }
}

#[derive(Debug, Clone)]
enum TileType {
    Empty,
    Mirror1,
    Mirror2,
    SplitterH,
    SplitterV,
}

impl From<char> for TileType {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '/' => Self::Mirror1,
            '\\' => Self::Mirror2,
            '-' => Self::SplitterH,
            '|' => Self::SplitterV,
            _ => panic!("In the disco!"),
        }
    }
}

#[derive(Debug, Clone)]
struct Tile {
    t: TileType,
    visited: HashSet<Direction>,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.t {
            TileType::Mirror1 => f.write_str("/"),
            TileType::Mirror2 => f.write_str("\\"),
            TileType::SplitterH => f.write_str("–"),
            TileType::SplitterV => f.write_str("|"),
            TileType::Empty => match self.visited.len() {
                0 => f.write_str("·"),
                1 => match self.visited.iter().next().unwrap() {
                    North => f.write_str("↑"),
                    Eastt => f.write_str("→"),
                    South => f.write_str("↓"),
                    Westt => f.write_str("←"),
                },
                x => f.write_str(&x.to_string())
            },
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        Self {
            t: TileType::from(value),
            visited: HashSet::new(),
        }
    }
}

#[derive(Clone)]
struct Grid(Vec<Vec<Tile>>);

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for tile in row {
                tile.fmt(f)?;
            }
            writeln!(f,"")?;
        }
        writeln!(f, "")
    }
}


#[aoc_generator(day16)]
fn parser(input: &str) -> Grid {
    Grid(input
        .lines()
        .map(|s| s.chars().map(Tile::from).collect())
        .collect())
}

fn solver(mut grid: Grid, start: ((i32, i32), Direction)) -> usize {
    let mut laszers: Vec<((i32, i32), Direction)> = vec![start];

    while let Some(laser) = laszers.pop() {
        let mut dir = laser.1;
        let pos: Option<(i32, i32)> = dir.step(laser.0);
        if pos.is_none() {
            continue;
        }
        let pos = pos.unwrap();
        let row: Option<&mut Vec<Tile>> = grid.0.get_mut(pos.0 as usize);
        if row.is_none() {
            continue;
        }
        let tile: Option<&mut Tile> = row.unwrap().get_mut(pos.1 as usize);
        if tile.is_none() {
            continue;
        }
        let tile = tile.unwrap();
        if tile.visited.contains(&dir) {
            continue;
        }
        tile.visited.insert(dir);

        match tile.t {
            TileType::Empty => laszers.push((pos, dir)),
            TileType::Mirror1 => {
                dir = match dir {
                    North => Eastt,
                    Eastt => North,
                    Westt => South,
                    South => Westt,
                };
                laszers.push((pos, dir))
            }
            TileType::Mirror2 => {
                dir = match dir {
                    North => Westt,
                    Westt => North,
                    Eastt => South,
                    South => Eastt,
                };
                laszers.push((pos, dir))
            }
            TileType::SplitterH => match dir {
                Eastt | Westt => laszers.push((pos, dir)),
                North | South => {
                    laszers.push((pos, Eastt));
                    laszers.push((pos, Westt));
                }
            },
            TileType::SplitterV => match dir {
                North | South => laszers.push((pos, dir)),
                Eastt | Westt => {
                    laszers.push((pos, North));
                    laszers.push((pos, South));
                }
            },
        }
    }

    return grid.0.iter().map(|row| row.iter().filter(|x| !x.visited.is_empty()).count()).sum();
}


#[aoc(day16, part1)]
fn solver_part1(grid: &Grid) -> usize {
    return solver(grid.clone(), ((0, -1), Eastt))
}


#[aoc(day16, part2, single)]
fn solver_part2_single(grid: &Grid) -> usize {
    let w: i32 = grid.0.first().unwrap().len() as i32;
    let h: i32 = grid.0.len() as i32;

    let mut starts: Vec<((i32, i32), Direction)> = Vec::new();
    for i in 0..h {
        starts.push(((i as i32, -1), Eastt));
        starts.push(((i as i32, w), Westt));
    }
    for i in 0..w {
        starts.push(((-1, i), South));
        starts.push(((h, i), North));
    }

    return starts.iter().map(|s| solver(grid.clone(), *s)).max().unwrap();
}

#[aoc(day16, part2, stupid)]
fn solver_part2_stupid_multi(grid: &Grid) -> usize {
    let w: i32 = grid.0.first().unwrap().len() as i32;
    let h: i32 = grid.0.len() as i32;

    let mut starts: Vec<((i32, i32), Direction)> = Vec::new();
    for i in 0..h {
        starts.push(((i as i32, -1), Eastt));
        starts.push(((i as i32, w), Westt));
    }
    for i in 0..w {
        starts.push(((-1, i), South));
        starts.push(((h, i), North));
    }

    let mut handles = vec![];

    for start in starts {
        let l_grid = grid.clone();
        let handle = std::thread::spawn(move || solver(l_grid, start));
        handles.push(handle);
    }

    let mut results = vec![];

    for handle in handles {
        results.push(handle.join().unwrap());
    }

    return *results.iter().max().unwrap();
}

#[aoc(day16, part2, multi)]
fn solver_part2_multi(grid: &Grid) -> usize {
    let grid = grid.clone();
    let w: i32 = grid.0.first().unwrap().len() as i32;
    let h: i32 = grid.0.len() as i32;

    let mut handles = vec![];

    let g = grid.clone();
    let handle = std::thread::spawn(move || {
        (0..h).map(|i| solver(g.clone(), ((i, -1), Eastt))).max().unwrap()
    });
    handles.push(handle);

    let g = grid.clone();
    let handle = std::thread::spawn(move || {
        (0..h).map(|i| solver(g.clone(), ((i, w), Westt))).max().unwrap()
    });
    handles.push(handle);

    let g = grid.clone();
    let handle = std::thread::spawn(move || {
        (0..w).map(|i| solver(g.clone(), ((-1, i), South))).max().unwrap()
    });
    handles.push(handle);

    let g = grid.clone();
    let handle = std::thread::spawn(move || {
        (0..w).map(|i| solver(g.clone(), ((h, i), North))).max().unwrap()
    });
    handles.push(handle);
    
    let mut results = vec![];

    for handle in handles {
        results.push(handle.join().unwrap());
    }

    return *results.iter().max().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = ".|...\\....\n|.-.\\.....\n.....|-...\n........|.\n..........\n.........\\\n..../.\\\\..\n.-.-/..|..\n.|....-|.\\\n..//.|....";

    #[test]
    fn test_solver_part1() {
        assert_eq!(5, solver_part1(&parser(EXAMPLE_1)));
    }
}


