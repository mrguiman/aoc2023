use std::vec::Vec;
use std::collections::VecDeque;
use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let input = String::from_utf8(std::fs::read("day3/input")?);
    if let Ok(contents) = input {
        println!("----");
        println!("Day 2");
        println!("Part 1: {}", compute_answer_1(&contents));
        println!("Part 2: {}", compute_answer_2(&contents));
        println!("----");
    }
    Ok(())
}

fn compute_answer_1(contents: &str) -> String {
    let mut connected_lines: VecDeque<&str> = VecDeque::new();

    let mut peekable_content = contents.lines().peekable();
    connected_lines.push_back(peekable_content.peek().unwrap_or(&""));

    let mut part_numbers: Vec<usize> = Vec::new();
    while let Some(line) = peekable_content.next() {
        connected_lines.push_back(peekable_content.peek().unwrap_or(&""));
        if connected_lines.len() > 3 { 
            connected_lines.pop_front(); 
        }

        find_symbol_adjacent_numbers(line, get_symbol_indices(connected_lines.clone()), &mut part_numbers);
    }
    format!("{}", part_numbers.iter().sum::<usize>())
}

fn compute_answer_2(contents: &str) -> String {
    let mut connected_lines: VecDeque<&str> = VecDeque::new();

    let mut peekable_content = contents.lines().peekable();
    connected_lines.push_back(peekable_content.peek().unwrap_or(&""));

    let mut gear_ratios: Vec<usize> = Vec::new();
    while let Some(line) = peekable_content.next() {
        connected_lines.push_back(peekable_content.peek().unwrap_or(&""));
        if connected_lines.len() > 3 { 
            connected_lines.pop_front(); 
        }

        gear_ratios.append(&mut find_gear_ratios(line, connected_lines.clone()));
    }
    format!("{}", gear_ratios.iter().sum::<usize>())
}

fn get_symbol_indices(strings: VecDeque<&str>) -> HashSet<usize> {
    let mut indices = HashSet::new();
    strings.iter().for_each(|str| {
        str.char_indices()
            .for_each(|(i, char)| {
                if is_symbol(char) {
                    indices.insert(i);
                }
            });
    });
    indices
}

fn find_symbol_adjacent_numbers(line: &str, symbol_indices: HashSet<usize>, part_numbers: &mut Vec<usize>) {
    let mut buffer = String::new();
    let mut char_iterator = line.char_indices().peekable();

    let mut check_number = |number_last_index, number_first_index: usize, number: String| {
        let number_is_adjacent = symbol_indices.iter()
            .any(|symbol_index| {
                *symbol_index >= number_first_index.saturating_sub(1) &&
                    *symbol_index <= number_last_index + 1
            });
            if number_is_adjacent {
                if let Ok(x) = number.parse::<usize>() {
                    part_numbers.push(x);
                }
            }
    };

    while let Some((i, char)) = char_iterator.next() {
        if char.is_ascii_digit() {
            buffer.push(char);
        } else {
            buffer = String::new();
        }

        if !buffer.is_empty() {
            let next_char = char_iterator.peek();
            let number_first_index = i.saturating_sub(buffer.len().saturating_sub(1));
            match next_char {
                Some((_, next_char)) => if !next_char.is_alphanumeric() { check_number(i, number_first_index, buffer.clone()) },
                None => check_number(i, number_first_index, buffer.clone())
            }
        }
    }
}

fn is_symbol(character: char) -> bool {
    !character.is_alphanumeric() && character != '.'
}

fn find_gear_ratios(line: &str, connected_lines: VecDeque<&str>) -> Vec<usize>{
    line.char_indices()
        .filter(|(_, c)| *c == '*')
        .filter_map(|(gear_index, _)| {
            let mut gear_adjacent_numbers: Vec<usize> = Vec::new();
            connected_lines.iter().for_each(|l| {
                find_symbol_adjacent_numbers(l, HashSet::from([gear_index]), &mut gear_adjacent_numbers);
            });

            if gear_adjacent_numbers.len() == 2 {
                return Some(gear_adjacent_numbers.iter().product::<usize>());
            }
            None
        })
        .collect()
}

