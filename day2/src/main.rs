use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let input = String::from_utf8(std::fs::read("day2/input")?);
    if let Ok(contents) = input {
        println!("----");
        println!("Day 2");
        println!("Part 1: {}", compute_answer_1(&contents));
        println!("Part 2: {}", compute_answer_2(&contents));
        println!("----");
    }
    Ok(())
}

struct Limits {
    blue: usize,
    red: usize,
    green: usize
}

fn compute_answer_1(contents: &str) -> usize {
    let limits = Limits { blue: 14, red: 12, green: 13 };
    let games_mapping = get_mapping_from_lines(contents);

    games_mapping.keys().fold(0, |score, game_number| {
        if let Some(color_draws) = games_mapping.get(game_number) {
            if *color_draws.get("blue").unwrap_or(&0) <= limits.blue &&
                *color_draws.get("red").unwrap_or(&0) <= limits.red &&
                    *color_draws.get("green").unwrap_or(&0) <= limits.green {
                        return score + game_number;
                    }
        }
        score
    })
}
fn compute_answer_2(contents: &str) -> usize {
    let games_mapping = get_mapping_from_lines(contents);
    games_mapping.keys().fold(0, |score, game_number| {
        if let Some(color_draws) = games_mapping.get(game_number) {
            return color_draws.values().product::<usize>() + score
        }
        score
    })
}
fn get_mapping_from_lines(contents: &str) -> HashMap<usize, HashMap<String, usize>> {
    contents.lines().fold(HashMap::new(), |mut acc, line| {
        let mut line_iter = line.split(':');
        acc.insert(
            parse_game_number(line_iter.next()).unwrap(),
            get_max_cubes_per_game(line_iter.next())
            );
        acc
    })
}

fn parse_game_number(opt_str: Option<&str>) -> Option<usize> {
    opt_str.map(|str| str.splitn(2, ' '))
        .and_then(Iterator::last)
        .map(str::parse::<usize>)
        .and_then(Result::ok)

}
fn parse_color_and_number(draw: &str) -> Option<(String, usize)> {
    let mut split = draw.rsplitn(2, ' ');
    let opt_color = split.next();
    let opt_draw = split.next().map(String::from);
    match (opt_color, opt_draw) {
        (Some(color), Some(number_of_cubes)) => {
            Some((
                    String::from(color),
                    number_of_cubes.parse::<usize>().unwrap_or(0)
                 ))
        },
        _ => None
    }
}

fn get_max_cubes_per_game(draws_as_str: Option<&str>) -> HashMap<String, usize> {
    draws_as_str.map(|str| str.trim().split(';'))
        .map(|draws| {
            draws.fold(HashMap::new(), |mut acc, draw| {
                draw.trim().split(',').for_each(|color_and_number_as_str| {
                    if let Some((color, number_of_cubes)) = parse_color_and_number(color_and_number_as_str.trim()) {
                        acc.insert(color.clone(), *std::cmp::max(&number_of_cubes, acc.get(&color).unwrap_or(&0)));
                    }
                });
                acc
            })
        }).unwrap_or_default()
}

