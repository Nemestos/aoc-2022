use std::collections::VecDeque;

use regex::Regex;

type Stacks = Vec<VecDeque<char>>;
pub fn captures_letters(line: &str) -> Vec<char> {
    let stack_regex = Regex::new(r"\[[A-Z]\]|    ").unwrap();

    let mut final_letters: Vec<char> = Vec::new();
    for capture in stack_regex.captures_iter(line) {
        if capture[0].starts_with(" ") {
            final_letters.push(' ');
            continue;
        }
        let letter = &capture[0].chars().nth(1).unwrap();
        final_letters.push(*letter);
    }
    final_letters
}
pub fn parse_stacks(header: &Vec<&str>) -> Stacks {
    let stack_height = header.len() - 1;
    let stack_count = header
        .last()
        .unwrap()
        .split("   ")
        .collect::<Vec<&str>>()
        .len();
    let mut stacks: Stacks = vec![VecDeque::default(); stack_count];

    for line in &header[..stack_height] {
        let letters = captures_letters(line);
        let mut stack_ind = 0;
        for letter in letters {
            if letter == ' ' {
                stack_ind += 1;
                continue;
            }
            stacks[stack_ind].push_back(letter);
            stack_ind += 1;
        }
    }
    stacks
}

pub fn move_from_stack(stacks: &mut Stacks, from: usize, to: usize, number: usize) {
    let mut temp_group: VecDeque<char> = [].into();
    for _ in 0..number {
        let letter = stacks[from].pop_front().unwrap();
        temp_group.push_front(letter)
    }
    for letter in temp_group {
        stacks[to].push_front(letter)
    }
}
pub fn get_message_from_stacks(stacks: &Stacks) -> String {
    let mut message = String::new();
    for stack in stacks {
        message.push(stack[0]);
    }
    message
}

pub fn main() {
    let input = include_str!("../input.txt");
    let header_body: Vec<&str> = input.split("\n\n").collect();
    let header: Vec<&str> = header_body[0].split("\n").collect();
    let body: Vec<&str> = header_body[1].split("\n").collect();
    let mut stacks = parse_stacks(&header);

    body.into_iter().for_each(|l| {
        let splitted: Vec<&str> = l.split(" ").collect();
        let number = splitted[1].parse::<usize>().unwrap();
        let from = splitted[3].parse::<usize>().unwrap();
        let to = splitted[5].parse::<usize>().unwrap();
        println!("{} {} {}", number, from, to);
        move_from_stack(&mut stacks, from - 1, to - 1, number)
    });

    let message = get_message_from_stacks(&stacks);
    println!("{}", message)
}
