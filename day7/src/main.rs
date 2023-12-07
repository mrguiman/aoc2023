use std::cmp::Ordering;
use std::str::FromStr;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let input = String::from_utf8(std::fs::read("day7/input")?);
    if let Ok(contents) = input {
        println!("----");
        println!("Day 7");
        println!("Part 1: {}", compute_answer_1(&contents));
        println!("Part 2: {}", compute_answer_2(&contents));
        println!("----");
    }
    Ok(())
}

fn compute_answer_1(contents: &str) -> String {
    let mut hands: Vec<Hand> = contents.lines()
        .map(Hand::from_str)
        .filter_map(Result::ok)
        .collect();

    sort_hands(&mut hands, get_card_strength);
    format!("{}", hands.iter().enumerate().fold(0, |acc, (i, hand)| {
        acc + (hand.bid * (i+1))
    }))
}

fn compute_answer_2(contents: &str) -> String {
    let mut hands: Vec<Hand> = contents.lines()
        .map(|line| {
            let mut hand = Hand::from_str(line).ok().unwrap();
            hand.apply_joker_rule();
            hand
        })
    .collect();


    sort_hands(&mut hands, get_joker_rule_card_strength);
    format!("{}", hands.iter().enumerate().fold(0, |acc, (i, hand)| {
        acc + (hand.bid * (i+1))
    }))
}

fn sort_hands(hands: &mut Vec<Hand>, card_to_strength: fn (char) -> usize) {
    hands.sort_by(|a,b| {
        match a.strength.cmp(&b.strength) {
            Ordering::Equal => split_tie(&a, &b, card_to_strength),
            x => x
        }
    });
}
fn split_tie(a: &Hand, b: &Hand, card_to_strength: fn (char) -> usize) -> Ordering {
    let mut iter = a.cards.chars().map(card_to_strength).zip(b.cards.chars().map(card_to_strength));
    let mut cards = iter.next().unwrap();
    loop {
        if cards.0.cmp(&cards.1) != Ordering::Equal {
            break;
        }
        cards = iter.next().unwrap()
    }
    cards.0.cmp(&cards.1)
}
fn get_joker_rule_card_strength(card: char) -> usize {
    "J23456789TQKA".chars().position(|c| c == card).unwrap()
}
fn get_card_strength(card: char) -> usize {
    "23456789TJQKA".chars().position(|c| c == card).unwrap()
}


#[derive(Debug)]
struct Hand {
    strength: usize,
    cards: String,
    cards_counts: HashMap<char, usize>,
    bid: usize
}
impl Hand {
    // TODO cleanup this hot mess
    fn apply_joker_rule(&mut self) {
        if let Some(joker_count) = self.cards_counts.get(&'J') {
            let mut new_counts = self.cards_counts.clone();
            new_counts.remove(&'J');

            let highest_card = new_counts.iter().fold(('J', 0), |acc, card| {
                if card.1 > &acc.1 { 
                    (*card.0, *card.1) 
                } else { 
                    acc 
                }
            });
            
            new_counts.entry(highest_card.0).and_modify(|c| {
                *c = std::cmp::min(5, *c+joker_count)
            }).or_insert(*joker_count);
            self.strength = get_hand_strength(&new_counts);
        }
    }
}
impl FromStr for Hand {
    type Err = ();
    fn from_str(string: &str) -> Result<Self, ()> {
        let (cards, bid) = string.split_once(' ').unwrap();
        let cards_counts = cards.chars()
            .fold(HashMap::new(), |mut acc, card| {
                acc.entry(card).and_modify(|e| *e += 1).or_insert(1usize);
                acc
            });

        Ok(Hand {
            strength: get_hand_strength(&cards_counts),
            cards: String::from(cards),
            bid: bid.parse::<usize>().unwrap(),
            cards_counts
        })
    }
}
fn get_hand_strength(cards_counts: &HashMap<char, usize>) -> usize {
    let mut counts: Vec<usize> = cards_counts
        .values()
        .map(|x| *x)
        .collect();
    counts.sort_by(|a, b| b.cmp(a));

    let mut iter = counts.iter();
    return match iter.next() {
        Some(5) => 7,
        Some(4) => 6,
        Some(3) => {
            let next_occurences = iter.next().unwrap();
            if *next_occurences == 2 {
                5
            } else {
                4
            }
        },
        Some(2) => {
            let next_occurences = iter.next().unwrap();
            if *next_occurences == 2 {
                3
            } else {
                2
            }
        },
        Some(1) => 1,
        _ => 0
    }
}
