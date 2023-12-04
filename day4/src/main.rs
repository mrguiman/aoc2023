use std::str::FromStr;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let input = String::from_utf8(std::fs::read("day4/input")?);
    if let Ok(contents) = input {
        println!("----");
        println!("Day 4");
        println!("Part 1: {}", compute_answer_1(&contents));
        println!("Part 2: {}", compute_answer_2(&contents));
        println!("----");
    }
    Ok(())
}

fn compute_answer_1(contents: &str) -> String {
    format!("{}", contents.lines()
        .map(Card::from_str)
        .filter_map(Result::ok)
        .map(Card::calculate_score)
        .sum::<usize>())
}

fn compute_answer_2(contents: &str) -> String {
    format!("{}", contents.lines()
        .map(Card::from_str)
        .filter_map(Result::ok)
        .fold(HashMap::new(), |mut card_counts, card| {
            // Scorecards are "copied" by means of increasing count by how many of the current card
            // we have
            let current_scorecard_count = *card_counts.entry(card.id).and_modify(|x| { *x += 1 }).or_insert(1);
            for j in 1..= card.get_matches().len() {
                card_counts
                    .entry(card.id + j)
                    .and_modify(|count| { *count += current_scorecard_count;})
                    .or_insert(current_scorecard_count);
            }
            card_counts
        })
        .values().sum::<usize>())
}
fn parse_numbers(number_list: &str) -> HashSet<usize> {
    number_list.split_whitespace()
        .map(str::parse::<usize>)
        .filter_map(Result::ok)
        .collect()
}

struct Card {
    id: usize,
    winning_numbers: HashSet<usize>,
    draw_numbers: HashSet<usize>
}
impl Card {
    pub fn calculate_score(card: Card) -> usize{
        card.score()
    }
    fn get_matches(&self) -> HashSet<&usize> {
        self.winning_numbers.intersection(&self.draw_numbers).collect()
    }
    fn score(&self)  -> usize {
        match self.get_matches().len() {
            match_count if match_count > 0 => 2usize.pow(match_count.saturating_sub(1) as u32),
            _ => 0
        }
    }
}
impl FromStr for Card {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if let Some((card_str, numbers_str)) = string.split_once(':') {
            let all_numbers = numbers_str.split_once('|')
                .map(|(win,draw)| (parse_numbers(win), parse_numbers(draw)));

            let card_number = card_str.split_whitespace().last()
                .map(str::parse::<usize>)
                .and_then(Result::ok);

            return match (card_number, all_numbers) {
                (Some(id), Some((winning_numbers, draw_numbers))) => Ok(
                    Card {
                        id,
                        winning_numbers,
                        draw_numbers
                    }
                    ),
                _ => Err(())
            }
        }

        Err(())
    }
}
