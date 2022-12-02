use phf::phf_map;

pub fn main() {
    static MAPPER: phf::Map<&'static str, &'static str> = phf_map! {
        "A"=>"X",
        "B"=>"Y",
        "C"=>"Z",

    };
    static RULES: phf::Map<&'static str, &'static str> = phf_map! {
        "X"=>"Z",
        "Y"=>"X",
        "Z"=>"Y",

    };
    static SCORES: phf::Map<&'static str, u32> = phf_map! {
        "X"=>1,
        "Y"=>2,
        "Z"=>3,
    };
    let lines = include_str!("../input.txt").split("\n");
    let scores = lines.map(|l| {
        let mut plays = l.split(" ");
        let first = MAPPER[plays.next().unwrap()];
        let second = plays.next().unwrap();
        let win_of_second = RULES[second];
        let final_score =
            SCORES[second] + 3 * (first == second) as u32 + 6 * (win_of_second == first) as u32;
        final_score
    });
    let scores_sum: u32 = scores.sum();
    println!("{}", scores_sum)
}
