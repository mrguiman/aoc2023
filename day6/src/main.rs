fn main() -> std::io::Result<()> {
    let input = String::from_utf8(std::fs::read("day6/input")?);
    if let Ok(contents) = input {
        println!("----");
        println!("Day 6");
        println!("Part 1: {}", compute_answer_1(&contents));
        println!("Part 2: {}", compute_answer_2(&contents));
        println!("----");
    }
    Ok(())
}

fn compute_answer_1(contents: &str) -> String {
    let lines: Vec<&str> = contents.lines().collect();
    let times: Vec<usize> = lines.get(0).unwrap().split_whitespace().skip(1).map(str::parse::<usize>).filter_map(Result::ok).collect();
    let distances: Vec<usize> = lines.get(1).unwrap().split_whitespace().skip(1).map(str::parse::<usize>).filter_map(Result::ok).collect();

    format!("{}", std::iter::zip(times, distances)
        .map(count_winning_hold_times)
        .product::<usize>())
}
fn compute_answer_2(contents: &str) -> String {
    let lines: Vec<&str> = contents.lines().collect();
    let race_time: usize = lines.get(0).unwrap().strip_prefix("Time: ").unwrap().replace(' ', "").parse().unwrap();
    let distance: usize = lines.get(1).unwrap().strip_prefix("Distance: ").unwrap().replace(' ', "").parse().unwrap();
    format!("{}", count_winning_hold_times((race_time, distance)))
}

fn count_winning_hold_times((race_time, distance): (usize, usize)) -> usize {
    (0..=race_time)
        .filter(|t| t * (race_time-t) > distance)
        .sum()
}
