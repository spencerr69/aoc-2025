use std::ops::RangeInclusive;
use std::{fs, path};

pub fn run() {
    let input = get_input();

    let ranges = parse_input(input);

    let flops = count_flops(ranges);

    #[cfg(debug_assertions)]
    println!("flops: {flops}");
}

pub fn get_input() -> String {
    let path = path::Path::new("src/inputs/2.txt");

    let input = fs::read_to_string(path).expect("Couldn't find file 2.txt");
    input
}

pub fn parse_input(input: String) -> Vec<RangeInclusive<i128>> {
    let ranges = input
        .split(",")
        .filter_map(|s| {
            let ranges: Vec<i128> = s
                .split("-")
                .map(|string| string.parse::<i128>())
                .filter_map(|result| result.ok())
                .collect();
            let [from, to] = ranges.as_slice() else {
                return None;
            };
            Some(from.clone()..=to.clone())
        })
        .collect();

    ranges
}

pub fn count_flops(ranges: Vec<RangeInclusive<i128>>) -> i128 {
    let flops = ranges.iter().fold(0 as i128, |mut acc, range| {
        acc = range.clone().fold(acc, |acc, password| {
            let password = password.to_string();
            if !password.len().is_multiple_of(2) {
                return acc;
            }

            let halfway = password.len() / 2;

            let (left_word, right_word) = (&password[0..halfway], &password[halfway..]);

            if !left_word.contains(right_word) {
                return acc;
            }

            acc + password.parse::<i128>().unwrap_or(0)
        });
        acc
    });
    flops
}
