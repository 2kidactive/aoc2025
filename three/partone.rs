use std::fs;

fn main() {
    let contents = fs::read_to_string("data.txt").expect("");
    let mut sum = 0;
    for bank in contents.lines() {
        let mut high = 1;
        let mut highb = 1;
        let mut index = 0;
        let len = bank.len();
        for c in (&bank[0..len]).chars() {
            index += 1;
            let joltage = c.to_digit(10).expect("");
            if joltage > high && index != len {
                high = joltage;
                highb = 1;
            } else if joltage > highb {
                highb = joltage;
            }
        }
        sum += high * 10 + highb;
    }
    println!("{sum}");
}
