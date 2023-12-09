use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let input = String::from_utf8(std::fs::read("day8/input")?);
    if let Ok(contents) = input {
        println!("----");
        println!("Day 8");
        println!("Part 1: {}", compute_answer_1(&contents));
        println!("Part 2: {}", compute_answer_2(&contents));
        println!("----");
    }
    Ok(())
}
fn compute_answer_1(contents: &str) -> String {
    let (instructions, network) = contents.split_once("\n\n").unwrap();
    let network_map = map_network(network);

    let mut current_node = network_map.get("AAA").unwrap();
    let mut counter = 0;
    let mut direction_iterator = instructions.chars().cycle();
    loop {
        let next_node_key = match direction_iterator.next().unwrap() {
            'L' => {
                &current_node.0
            },
            'R' => {
                &current_node.1
            }
            _ => { panic!("Your direction means nothing to me") }
        };
        counter += 1;
        if next_node_key == "ZZZ"{
            break;
        } else {
            current_node = network_map.get(next_node_key).unwrap();
        }

    }
    counter.to_string()
}

fn compute_answer_2(contents: &str) -> String {
    let (instructions, network) = contents.split_once("\n\n").unwrap();
    let network_map = map_network(network);

    let nodes: HashMap<&String, &(String, String)> = network_map.iter()
        .filter(|entry| entry.0.chars().last().unwrap() == 'A')
        .collect();
    let counts: Vec<usize> = nodes.iter().map(|node| {
        let mut direction_iterator = instructions.chars().cycle();
        let mut counter = 0;
        let mut crawl_node = *node.1;
        loop {
            counter += 1;
            let direction = direction_iterator.next().unwrap();
            let next_node_key: &String = match direction {
                'L' => {
                    &crawl_node.0

                },
                'R' => {
                    &crawl_node.1
                }
                _ => { panic!("Your direction means nothing to me") }
            };
            if next_node_key.chars().last().unwrap() == 'Z' {
                return counter;
            }
            crawl_node = network_map.get(next_node_key).unwrap();
        }
    })
    .collect();

    counts.iter().fold(1, |acc, c| lcm(acc, *c)).to_string()
}

fn gcd(a: usize, b: usize) -> usize {
    match b {
        0 => a,
        _ => gcd(b, a%b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a*b / gcd(a,b)
}

fn map_network(network_str: &str) -> HashMap<String, (String, String)> {
    network_str.lines().fold(HashMap::new(), |mut acc, node_str| {
        let (node, linked_nodes) = node_str.split_once(" = ").unwrap();
        let linked_node_values: Vec<String> = linked_nodes.replace('(', "").replace(')', "").split(", ").map(String::from).collect();

        acc.insert(
            String::from(node),
            (linked_node_values.get(0).unwrap().clone(), linked_node_values.get(1).unwrap().clone())
            );
        acc
    })
}
