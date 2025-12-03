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

            let mut factors = list_factors(password.len() as i128);

            factors.pop();

            let mut pattern = &password[0..factors.pop().unwrap_or(0) as usize];

            while !factors.is_empty() {
                if password.replace(pattern, "").len() == 0 {
                    return acc + password.parse::<i128>().unwrap_or(0);
                } else {
                    if let Some(next_factor) = factors.pop() {
                        pattern = &pattern[0..next_factor as usize];
                    } else {
                        return acc;
                    }
                }
            }

            acc
        });
        acc
    });
    flops
}

fn list_factors(number: i128) -> Vec<i128> {
    let mut factors: Vec<i128> = Vec::new();
    let mut i: i128 = 1;
    while i * i <= number {
        if number % i == 0 {
            factors.push(i);
            if i * i != number {
                factors.push(number / i);
            }
        }
        i += 1;
    }
    factors.sort();
    return factors;
}
