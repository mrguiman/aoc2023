use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let input = String::from_utf8(std::fs::read("day9/input")?);
    if let Ok(contents) = input {
        println!("----");
        println!("Day 9");
        println!("Part 1: {}", compute_answer_1(&contents));
        println!("Part 2: {}", compute_answer_2(&contents));
        println!("----");
    }
    Ok(())
}
fn compute_answer_1(contents: &str) -> String {
    contents.lines()
        .map(generate_sequences)
        .map(get_end_extrapolated_numbers)
        .map(find_next_history_value)
        .sum::<isize>().to_string()
}

fn compute_answer_2(contents: &str) -> String {
    contents.lines()
        .map(generate_sequences)
        .map(get_beginning_extrapolated_numbers)
        .map(find_next_history_value)
        .sum::<isize>().to_string()
}

fn generate_sequences(line: &str) -> Vec<Vec<isize>> {
    let line_numbers: Vec<isize> = line.split_whitespace()
        .filter_map(|x| x.parse::<isize>().ok())
        .collect();
    let mut sequences: Vec<Vec<isize>> = vec!(line_numbers.clone());

    gen_next_lines(line_numbers, &mut sequences);
    sequences
}

fn gen_next_lines(numbers: Vec<isize>, sequences: &mut Vec<Vec<isize>>) {
    let next: Vec<isize> = numbers.as_slice()
        .windows(2)
        .map(|w| w[1] - w[0])
        .collect();

    sequences.push(next.clone());
    if next.iter().sum::<isize>() != 0 { 
        gen_next_lines(next, sequences)
    }
}

fn get_beginning_extrapolated_numbers(sequences: Vec<Vec<isize>>) -> Vec<isize> {
    let mut previous_number = 0;
    
    sequences.iter()
        .rev()
        .map(|sequence| {
            previous_number = sequence.first().unwrap() - previous_number;
            previous_number 
        })
        .collect()
}
fn get_end_extrapolated_numbers(sequences: Vec<Vec<isize>>) -> Vec<isize> {
    let mut previous_number = 0;
    
    sequences.iter()
        .rev()
        .map(|sequence| {
            previous_number = previous_number + sequence.last().unwrap();
            previous_number 
        })
        .collect()
}

fn find_next_history_value(extrapolated_numbers: Vec<isize>) -> isize {
    *extrapolated_numbers.last().unwrap()
}
