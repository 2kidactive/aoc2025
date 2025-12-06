use std::fs;

fn main() {
    let contents = fs::read_to_string("data.txt").expect("");
    let mut fresh: Vec<u64> = Vec::new();
    let mut check: Vec<u64> = Vec::new();
    let mut next_section = false;
    for line in contents.lines() {
        if next_section {
            check.push(line.trim().parse::<u64>().unwrap());
            println!("{line}");
        }
        if line.trim().is_empty() {
            next_section = true;
        }
        if !next_section {
            let (start, end) = line.split_once('-').expect("");
            fresh.push(start.parse::<u64>().unwrap());
            fresh.push(end.parse::<u64>().unwrap());
        }
    }
    let mut ans = 0;
    for i in check {
        for j in 0..fresh.len()-1 {
            if j % 2 == 0 {
                if i >= fresh[j] && i <= fresh[j+1] {
                    ans += 1;
                    break;
                }
            }
        }
    }
    println!("{ans}");
}
