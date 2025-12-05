use std::fs;
use std::cmp::max;

fn main() {
    let contents = fs::read_to_string("data.txt").expect("");
    let mut fresh: Vec<(u64, u64)> = Vec::new();
    for line in contents.lines() {
        if line.trim().is_empty() {
            break;
        }
        let (start, end) = line.split_once('-').expect("");
        fresh.push((start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()));
    }
    fresh.sort();
    let mut sum = 0;
    for i in (0..fresh.len()-1).rev() {
        let (a, b) = fresh[i];
        let (m, n) = fresh[i+1];
        // println!("{a} {b}, {m} {n}");
        if m > b { sum += b - a + 1; continue; }
        if sum != 0 { sum -= n - m + 1; }
        sum += max(b, n) - a + 1;
        fresh[i].1 = max(b,n);
        // println!("{sum}");
    }
    println!("{sum}");
}
