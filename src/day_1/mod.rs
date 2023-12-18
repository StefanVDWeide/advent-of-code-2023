use std::collections::HashMap;

use crate::utils::get_clean_input;

pub fn solution_1() {
    let input = get_clean_input("day_1").unwrap();
    let string_vec: Vec<&str> = input.split("\n").collect();
    let mut sum: i32 = 0;

    for line in string_vec {
        let mut number_string: String = "".to_owned();
        for c in line.chars() {
            if let Some(_) = c.to_digit(10) {
                number_string.push(c);
            }
        }
        if number_string.len() == 1 {
            let first_char = number_string.chars().nth(0).unwrap();
            number_string.push(first_char);
        } else if number_string.len() > 2 {
            let first_char = number_string.chars().nth(0).unwrap();
            let last_char = number_string.chars().last().unwrap();
            let mut new_string: String = "".to_owned();
            new_string.push(first_char);
            new_string.push(last_char);
            number_string = new_string;
        }

        let value: i32 = number_string.parse().unwrap();
        sum += value;
    }
    println!("Total sum part 1 {}", sum);
}

pub fn solution_2() -> i32 {
    let input = get_clean_input("day_1").unwrap();
    let string_vec: Vec<&str> = input.split("\n").collect();

    let number_map = HashMap::from([
        ("1", "1"),
        ("one", "1"),
        ("2", "2"),
        ("two", "2"),
        ("3", "3"),
        ("three", "3"),
        ("4", "4"),
        ("four", "4"),
        ("5", "5"),
        ("five", "5"),
        ("6", "6"),
        ("six", "6"),
        ("7", "7"),
        ("seven", "7"),
        ("8", "8"),
        ("eight", "8"),
        ("9", "9"),
        ("nine", "9"),
        ("10", "10"),
        ("ten", "10"),
    ]);

    let mut sum = 0;
    for line in &string_vec {
        let mut digit_indices = Vec::new();

        for (word, &num_str) in &number_map {
            let mut start = 0;
            while let Some(idx) = line[start..].find(word) {
                let actual_idx = start + idx;
                digit_indices.push((actual_idx, num_str));
                start = actual_idx + word.len();
            }
        }

        // Sort by index
        digit_indices.sort_by_key(|&(idx, _)| idx);

        if let Some(&(first_idx, first_num_str)) = digit_indices.first() {
            let last_num_str = digit_indices
                .last()
                .unwrap_or(&(first_idx, first_num_str))
                .1;
            sum += format!("{}{}", first_num_str, last_num_str)
                .parse::<i32>()
                .unwrap();
        }
    }

    println!("Total sum: {}", sum);
    sum
}
