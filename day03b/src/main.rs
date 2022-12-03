pub fn main() {
    let lines = include_str!("../input.txt").split("\n");
    let both_letters_sum: u32 = lines
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|l| {
            let first_part = &mut l[0].chars();
            let second_part = l[1];
            let third_part = l[2];
            let both = first_part
                .find(|e| {
                    second_part.contains(&e.to_string()) && third_part.contains(&e.to_string())
                })
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
