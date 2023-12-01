fn main() -> std::io::Result<()> {
    let input = String::from_utf8(std::fs::read("day1/input")?);
    if let Ok(contents) = input {
        let answer = contents.split("\n")
            .fold(0, |acc, line| {
                let target_digits = (
                    find_next_digit_as_number_or_letters(line.chars().collect::<Vec<char>>()),
                    find_next_digit_as_number_or_letters(line.chars().rev().collect::<Vec<char>>())
                    );
                match target_digits {
                    (Some(first_digit), Some(second_digit)) => {
                        return acc + String::from_iter([first_digit, second_digit]).parse::<i32>().unwrap_or(0)
                    },
                    _ => return acc
                };
            });
        println!("{}", answer);
    }
    return Ok(())
}

const DIGITS_IN_LETTERS: [&'static str; 18] = ["one", "eno", "two", "owt", "three", "eerht", "four", "ruof", 
      "five", "evif", "six", "xis", "seven", "neves", "eight", "thgie", "nine", "enin"];

fn find_next_digit_as_number_or_letters(chars: Vec<char>) -> Option<char> {
    let mut crawled_chars = String::new();
    let mut pointer = 0;

    while let Some(char) = chars.get(pointer) {
        if char.is_digit(10) {
            return Some(*char)
        }

        crawled_chars.push(*char);
        if DIGITS_IN_LETTERS.contains(&crawled_chars.as_str()) {
            return convert_digit_from_letters(crawled_chars)
        }

        let partial_digits_as_letters = DIGITS_IN_LETTERS.into_iter()
            .map(|str| str.get(0..crawled_chars.len()))
            .flatten()
            .collect::<Vec<&str>>();

        if !partial_digits_as_letters.contains(&crawled_chars.as_str()) {
            pointer -= crawled_chars.len() - 1;
            crawled_chars = String::new();
        }

        pointer += 1;

    }
    return None
}

fn convert_digit_from_letters(letters: String) -> Option<char> {
    match letters.as_str() {
        "one" | "eno" => Some('1'),
        "two" | "owt" => Some('2'),
        "three" | "eerht" => Some('3'),
        "four" | "ruof" => Some('4'),
        "five" | "evif" => Some('5'),
        "six" | "xis" => Some('6'),
        "seven" | "neves" => Some('7'),
        "eight" | "thgie" => Some('8'),
        "nine" | "enin" => Some('9'),
        _ => None
    }
}
