pub fn is_overlap(a:u32,b:u32,c:u32,d:u32)->bool{
    a<=d &&  b>=c
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


            is_overlap(a, b, c, d) as u32
        })
        .sum();
    println!("{}", range_sum);
}
