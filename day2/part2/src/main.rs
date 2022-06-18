use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("file not found");
    let lines = contents.lines();

    let mut aim = 0;
    let mut depth = 0;
    let mut horizontal_position = 0;

    for line in lines {
        let split_line: Vec<&str> = line.split(' ').collect();

        match split_line[..split_line.len()] {
            ["forward", distance, ..] => {
                let parsed_distance: i32 = distance.parse().unwrap();
                horizontal_position = horizontal_position + parsed_distance;
                depth = depth + (aim * parsed_distance);
            }
            ["up", distance, ..] => {
                let parsed_distance: i32 = distance.parse().unwrap();
                aim = aim - parsed_distance;
            }
            ["down", distance, ..] => {
                let parsed_distance: i32 = distance.parse().unwrap();
                aim = aim + parsed_distance;
            }
            _ => println!("no match"),
        }
    }

    println!(
        "h pos: {}, depth: {}, aim: {}",
        horizontal_position, depth, aim
    );
    println!("exact pos: {}", horizontal_position * depth);
}
