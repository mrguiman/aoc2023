use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let input = String::from_utf8(std::fs::read("day4/input")?);
    if let Ok(contents) = input {
        println!("----");
        println!("Day 4");
        println!("Part 1: {}", compute_answer_1(&contents));
        println!("----");
    }
    Ok(())
}

fn compute_answer_1(contents: &str) -> String {
    format!("{}", contents.lines()
            .map(|line| {
                println!("{}", line);
                if let Some((_, number_lists)) = line.split_once(':') {
                    return get_matches_score(number_lists)
                }
                0
            })
            .sum::<usize>())
}

fn get_matches_score(number_lists: &str) -> usize {
    if let Some((winning_numbers_str, owned_numbers_str)) = number_lists.split_once('|') {
        let (winning_numbers, owned_numbers) = (parse_numbers(winning_numbers_str), parse_numbers(owned_numbers_str));

        let matches: Vec<&usize> = winning_numbers.intersection(&owned_numbers).collect();
        if matches.len() >= 1 {
            return 1 * ((2 as usize).pow((matches.len().saturating_sub(1)) as u32))
        }
    }
    0
}

fn parse_numbers(number_list: &str) -> HashSet<usize> {
    number_list.trim().split_whitespace()
        .map(str::parse::<usize>)
        .filter_map(Result::ok)
        .collect()
}
