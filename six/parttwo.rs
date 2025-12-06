use std::fs;

fn main() {
    let contents = fs::read_to_string("data.txt").expect("");
    let mut vals: Vec<&str> = Vec::new();
    for line in contents.lines() {
        vals.push(line);
    }
    let mut opers: Vec<(char, usize)> = Vec::new();
    let last_row = vals[vals.len()-1].chars();
    let mut last_i = 0;
    let mut last_v = ' ';
    for (i, v) in last_row.clone().enumerate() {
        if v != ' ' {
            if i > 0 { opers.push((last_v, i - last_i - 1)); }
            last_i = i;
            last_v = v;
        }
        if i == last_row.clone().count()-1 { opers.push((last_v, i - last_i + 1)); }
    }
    let vals: Vec<Vec<char>> = vals.into_iter().map(|x| x.chars().collect()).collect();
    let mut numbers: Vec<u64> = Vec::new();
    for col in 0..vals[0].len() {
        let mut num = "".to_string();
        for row in 0..vals.len()-1 {
            let val = vals[row][col];
            num.push(val);
        }
        let ans: u64;
        if !num.trim().is_empty() { ans = num.trim().parse::<u64>().expect(""); }
        else { continue; }
        numbers.push(ans);
    }
    let mut base = 0;
    let mut ans = 0;
    for (oper, range) in opers {
        let mut sum = match oper {
            '*' => 1,
            _ => 0,
        };
        for i in base..base+range {
            match oper {
                '+' => sum += numbers[i],
                '*' => sum *= numbers[i],
                _ => sum += 0,
            };
        }
        base += range;
        ans += sum;
    }
    println!("{ans}");
}
