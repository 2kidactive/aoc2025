use std::fs;
use std::convert::TryInto;

fn main() {
    let contents = fs::read_to_string("data.txt").expect("");
    let mut sum = 0;
    for bank in contents.lines() {
        let mut index = 0;
        let len = bank.len();
        for n in 1..13 {
            let (joltage, i) = get_high(&bank[index..len-(12-n)]);
            index += i;
            let base: u64 = 10;
            sum += joltage * base.pow((12 - n).try_into().unwrap());
        }
    }
    println!("{sum}");
}

fn get_high(bank: &str) -> (u64, usize) {
    let mut high = 1;
    let mut i = 0;
    let mut index = 0;
    for c in bank.chars() {
        i += 1;
        let joltage = c.to_digit(10).expect("");
        if joltage > high { high = joltage; index = i; }
    }
    (high.into(), index)
}
