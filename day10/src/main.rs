use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let input = String::from_utf8(std::fs::read("day9/input")?);
    if let Ok(contents) = input {
        println!("----");
        println!("Day 10");
        println!("Part 1: {}", compute_answer_1(&contents));
        println!("Part 2: {}", compute_answer_2(&contents));
        println!("----");
    }
    Ok(())
}
fn compute_answer_1(contents: &str) -> String {
    String::new()
}

fn compute_answer_2(contents: &str) -> String {
    String::new()
}

enum Tile {
    Vertical, // Connects to top and bottom tile
    SouthWest, // Connects to left and bottom
    SouthEast, // Connects to right and bottom
    Horizontal,
    NorthEast,
    NorthWest,
    Ground
}

impl Tile {
    pub fn from(c: char) -> Tile {
        match c {
            '|' => Tile::Vertical,
            '-' => Tile::Horizontal,
            'L' => Tile::NorthEast,
            'J' => Tile::NorthWest,
            '7' => Tile::SouthWest,
            'F' => Tile::SouthEast,
            _ => Tile::Ground
        }
    }
    pub fn from_surroundings(top: Tile, right: Tile, bottom: Tile, left: Tile) {
        let top_connects = Tile::Vertical | Tile::SouthWest | Tile::SouthEast;
        match (top, right, bottom, left) {
            (top_connects, right_connnects) =>
        }
    }
}

struct Network {
    start: (usize, usize),
    tiles: Vec<Vec<Tile>>
}
impl Network {
    fn calculate_distance(x: (usize, usize), y: (usize, usize)) {
    }
}

