use std::{collections::HashSet, slice::Windows};

pub fn check_window(window: &mut Windows<char>) -> bool {
    let letters = &window.next().unwrap().to_vec();

    let mut unique = HashSet::new();
    letters.into_iter().for_each(|l| {
        unique.insert(l);
    });
    unique.len() == letters.len()
}

pub fn main() {
    let input = include_str!("../input.txt");

    let chars = input.chars().collect::<Vec<char>>();

    let mut window = chars.windows(4);
    let mut before_marker = 4;

    while !check_window(&mut window) {
        before_marker += 1
    }
    println!("{}", before_marker)
}
