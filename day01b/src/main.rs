pub fn main() {
    let lines = include_str!("../input.txt").split("\n\n");
    let mut mapped: Vec<u32> = lines
        .map(|a| {
            a.split("\n")
                .map(|a| a.parse::<u32>().unwrap())
                .reduce(|x, y| x + y)
                .unwrap()
        })
        .collect::<Vec<u32>>();
    mapped.sort_by(|a, b| b.cmp(a));
    let max: u32 = mapped.into_iter().take(3).sum();
    println!("{:?}", max);
}
