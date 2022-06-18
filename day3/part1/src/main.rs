use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("file not found");
    let lines: Vec<&str> = contents.lines().map(|line| line).collect();

    let line_length = lines[0].len();

    let mut zeroes = vec![0; line_length];
    let mut ones = vec![0; line_length];

    for binary in lines {
        let split_binary: Vec<char> = binary.chars().collect();
        for (i, value) in split_binary.iter().enumerate() {
            match value {
                '0' => zeroes[i] = zeroes[i] + 1,
                '1' => ones[i] = ones[i] + 1,
                _ => println!("No match"),
            }
        }
    }

    let mut gamma = vec!["0"; line_length];
    let mut epsilon = vec!["0"; line_length];

    for i in 0..line_length {
        if zeroes[i] > ones[i] {
            gamma[i] = "0";
            epsilon[i] = "1";
        } else if zeroes[i] < ones[i] {
            gamma[i] = "1";
            epsilon[i] = "0";
        }
    }

    let gamma_str = gamma.join("").to_owned();
    let epsilon_str = epsilon.join("").to_owned();

    // String -> &str to use i64::from_str_radix
    let g_str = &gamma_str[..];
    let e_str = &epsilon_str[..];

    let g_base_10 = i64::from_str_radix(g_str, 2).unwrap();
    let e_base_10 = i64::from_str_radix(e_str, 2).unwrap();

    println!("{} {} {}", g_base_10, e_base_10, g_base_10 * e_base_10);
}
