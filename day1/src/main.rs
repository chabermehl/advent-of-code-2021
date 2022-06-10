use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("file not found");
    let mut lines = contents.lines();

    // first value is the default min
    let mut current_min: i32 = lines.next().unwrap().parse().unwrap();
    let mut current_inc_count = 0;

    for l in lines {
        let parsed_line: i32 = l.parse().unwrap();

        if parsed_line > current_min {
            current_inc_count += 1;
            current_min = parsed_line;
        } else {
            current_min = parsed_line;
        }
    }

    println!("inc count: {}", current_inc_count);
}
