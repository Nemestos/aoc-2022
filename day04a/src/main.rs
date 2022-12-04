pub fn is_contains(a:u32,b:u32,c:u32,d:u32)->bool{
    a<=c && b>=d
}

pub fn main() {
    let lines = include_str!("../input.txt").split("\n");
    let range_sum: u32 = lines
        .map(|l| {
            let mut splitted = l.split(",");
            let mut first = splitted.next().unwrap().split("-");
            let mut second = splitted.next().unwrap().split("-");
            let a:u32 = first.next().unwrap().parse().unwrap();
            let b:u32 = first.next().unwrap().parse().unwrap();
            let c:u32 = second.next().unwrap().parse().unwrap();
            let d:u32 = second.next().unwrap().parse().unwrap();


            (is_contains(a, b, c, d) || is_contains(c, d, a, b)) as u32
        })
        .sum();
    println!("{}", range_sum);
}
