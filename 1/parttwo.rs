use std::fs;

fn main() {
    let contents = fs::read_to_string("data.txt").expect("");
    let mut pos: i16 = 50;
    let mut ans: i16 = 0;
    for line in contents.lines() {
        if line.is_empty() { break; }
        let (dir, amtstr) = line.split_at(1);
        let amt = amtstr.parse::<i16>().unwrap();
        match dir {
            "R" => pos += amt,
            "L" => pos -= amt,
            _ => panic!("Invalid Input!")
        };
        ans += pos.abs() / 100;
        if pos.abs() != amt && pos <= 0 { ans += 1; }
        pos = pos.rem_euclid(100);
    }
    println!("{ans}");
}
