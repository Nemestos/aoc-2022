use std::ops::RangeBounds;

pub fn main() {
    let lines = include_str!("../input.txt").split("\n");
    let both_letters_sum: u32 = lines
        .map(|l| {
            let len = l.len();
            let first_part = &mut l[..len / 2].chars();
            let second_part = &l[len / 2..].to_string();
            let both = first_part
                .find(|e| second_part.contains(&e.to_string()))
                .unwrap();
            let value = both.to_ascii_lowercase() as u32;
            let mut score = value - 96;
            if both.is_ascii_uppercase() {
                score += 26;
            }
            score
        })
        .sum();
    println!("{}", both_letters_sum);
}
