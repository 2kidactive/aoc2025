use std::fs;

fn main() {
    let contents = fs::read_to_string("data.txt").expect("");
    let mut vals: Vec<Vec<&str>> = Vec::new();
    for line in contents.lines() {
        let mut row: Vec<&str> = Vec::new();
        for val in line.split_whitespace() {
            row.push(val);
        }
        vals.push(row);
    }
    let mut ans = 0;
    for n in 0..vals[0].len() {
        let mut sum = 0;
        if vals[vals.len()-1][n] == "*" {
            sum +=1;
        }
        for m in 0..vals.len()-1 {
            let test = vals[m][n];
            println!("{test}");
            match vals[vals.len()-1][n] {
                "+" => sum += vals[m][n].parse::<u64>().expect(""),
                "*" => sum *= vals[m][n].parse::<u64>().expect(""),
                _ => (),
            };
            println!("SUM: {sum}");
        }
        ans += sum;
    }
    println!("{ans}");
}
