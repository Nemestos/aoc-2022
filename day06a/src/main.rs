use std::{collections::HashSet, slice::Windows};

const WINDOW_SIZE: usize = 14;
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

    let mut window = chars.windows(WINDOW_SIZE);
    let mut before_marker = WINDOW_SIZE;

    while !check_window(&mut window) {
        before_marker += 1
    }
    println!("{}", before_marker)
}
