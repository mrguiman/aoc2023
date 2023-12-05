use rayon::prelude::*;
use std::ops::Range;
use std::str::FromStr;
use std::collections::VecDeque;

fn main() -> std::io::Result<()> {
    let input = String::from_utf8(std::fs::read("day5/input")?);
    if let Ok(contents) = input {
        println!("----");
        println!("Day 5");
        println!("Part 1: {}", compute_answer_1(&contents));
        println!("Part 2: {}", compute_answer_2(&contents));
        println!("----");
    }
    Ok(())
}

fn compute_answer_1(contents: &str) -> String {
    let mut blocks: VecDeque<&str> = contents.split("\n\n").collect();

    let seeds = blocks.pop_front().unwrap_or("").split_whitespace()
        .map(str::parse::<usize>)
        .filter_map(Result::ok);
    let mappings: Vec<Mapping> = blocks.iter().map(|x| Mapping::from_str(x)).filter_map(Result::ok).collect();

    format!("{}", seeds
            .map(|x| crawl(x, &mappings))
            .min()
            .unwrap())
}
fn compute_answer_2(contents: &str) -> String {
    let mut blocks: VecDeque<&str> = contents.split("\n\n").collect();


    let seeds = blocks.pop_front().unwrap_or("").split_whitespace()
        .map(str::parse::<usize>)
        .filter_map(Result::ok)
        .collect::<Vec<usize>>();
    let mappings: Vec<Mapping> = blocks.iter().map(|x| Mapping::from_str(x)).filter_map(Result::ok).collect();

    format!("{}", seeds
            .as_slice()
            .par_chunks(2)
            .flat_map(|seeds| (seeds[0]..seeds[0]+seeds[1]))
            .map(|x|  crawl(x, &mappings))
            .min().unwrap())
}

fn crawl(seed_value: usize, mappings: &[Mapping]) -> usize {
    mappings.iter().fold(seed_value, |value, mapping| {
        let matching_range_opt = mapping.ranges.iter().find(|range| {
            range.source_range().contains(&value)
        });
        if let Some(range) = matching_range_opt {
            range.destination_start + (value - range.source_start)
        } else {
            value
        }
    })
}

#[derive(Debug)]
struct MappingRange {
    source_start: usize,
    destination_start: usize,
    step: usize
}
impl FromStr for MappingRange {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let components: Vec<usize> = string.split_whitespace()
            .map(str::parse::<usize>)
            .filter_map(Result::ok)
            .collect();

        Ok(MappingRange{
            source_start: *components.get(1).unwrap(),
            destination_start: *components.first().unwrap(),
            step: *components.get(2).unwrap()
        })
    }
}
impl MappingRange {
    fn source_range(&self) -> Range<usize> {
        self.source_start..self.source_start+self.step
    }
}

#[derive(Debug)]
struct Mapping {
    ranges: Vec<MappingRange>
}
impl FromStr for Mapping {
    type Err = ();
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let splits: VecDeque<&str> = string.split('\n').skip(1).filter(|x| !x.is_empty()).collect();
        Ok(Mapping {
            ranges: splits.iter().map(|x| MappingRange::from_str(x)).filter_map(Result::ok).collect()
        })
    }
}
