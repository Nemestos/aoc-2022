pub fn main() {
    let lines = include_str!("../input.txt").split("\n\n");
    let max: u32 = lines
        .map(|a| {
            a.split("\n")
                .map(|a| a.parse::<u32>().unwrap())
                .reduce(|x, y| x + y)
                .unwrap()
        })
        .max()
        .unwrap();
    println!("{}", max);
}
