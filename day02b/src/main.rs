use phf::phf_map;

pub fn main() {
    static MAPPER: phf::Map<&'static str, &'static str> = phf_map! {
        "A"=>"X",
        "B"=>"Y",
        "C"=>"Z",

    };
    static RULES_WIN: phf::Map<&'static str, &'static str> = phf_map! {
        "X"=>"Z",
        "Y"=>"X",
        "Z"=>"Y",

    };
    static RULES_LOSES: phf::Map<&'static str, &'static str> = phf_map! {
        "Z"=>"X",
        "X"=>"Y",
        "Y"=>"Z",

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
        let final_play = match second {
            "X" => RULES_WIN[first],
            "Y" => first,
            "Z" => RULES_LOSES[first],
            _ => "",
        };
        let win_of_second = RULES_WIN[final_play];
        let final_score = SCORES[final_play]
            + 3 * (first == final_play) as u32
            + 6 * (win_of_second == first) as u32;
        final_score
    });
    let scores_sum: u32 = scores.sum();
    println!("{}", scores_sum)
}
