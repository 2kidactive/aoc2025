use std::fs;

fn main() {
    let contents = fs::read_to_string("data.txt").expect("");
    let mut pos: i16 = 50;
    let mut ans: u16 = 0;
    for line in contents.lines() {
        if line.is_empty() { break; }
        let (dir, amt) = line.split_at(1);
        match dir {
            "R" => pos += amt.parse::<i16>().unwrap(),
            "L" => pos -= amt.parse::<i16>().unwrap(),
            _ => panic!("Invalid Input!")
        };
        pos = pos.rem_euclid(100);
        if pos == 0 { ans += 1; }
    }
    println!("{ans}");
}
