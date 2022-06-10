use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("file not found");
    let lines = contents.lines();

    let lines_arr: Vec<i32> = lines.map(|line| line.parse().unwrap()).collect();

    let mut count = 0;

    for (i, _) in lines_arr.iter().enumerate() {
        if i <= lines_arr.len() - 4 {
            let prev_value: i32 = lines_arr[i..i + 3].iter().sum();
            let next_value: i32 = lines_arr[i + 1..i + 4].iter().sum();

            if next_value > prev_value {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
