use std::fs;

fn main() {
    let contents = fs::read_to_string("data.txt").expect("");
    let mut sum = 0;
    for range in contents.split(',') {
        if range.trim().is_empty() { break; }
        let (start, end) = range.split_once('-').expect("");
        for n in start.parse::<u64>().unwrap()..end.parse::<u64>().unwrap()+1 {
            let num = n.to_string();
            let len = num.len() / 2;
            if num[0..len] == num[len..] {
                sum += n;
            }
        }
    }
    println!("{sum}");
}
