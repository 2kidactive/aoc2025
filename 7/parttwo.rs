use std::fs;
use std::cmp;

fn main() {
    let contents = fs::read_to_string("data.txt").expect("");
    let mut vals: Vec<Vec<u64>> = Vec::new();
    for line in contents.lines() {
        let mut row: Vec<u64> = Vec::new();
        for c in line.chars() {
            match c {
                '^' => row.push(1),
                _ => row.push(0),
            };
        }
        vals.push(row);
    }
    let mut max = 0;
    for (i, row) in vals.clone().into_iter().enumerate().rev() {
        for (j, val) in row.into_iter().enumerate() {
            match val {
                1 => vals[i][j] = calculate_splitter(&vals, (i, j)),
                _ => continue,
            };
            max = cmp::max(max, vals[i][j]);
        }
    }
    println!("{max}");
}

fn calculate_splitter(vals: &Vec<Vec<u64>>, index: (usize, usize)) -> u64 {
    let mut sum = 0;
    let mut done: (bool, bool) = (false, false);
    for i in index.0..vals.len() {
        if !done.0 && vals[i][index.1-1] != 0 { sum += vals[i][index.1-1]; done.0 = true; }
        if !done.1 && vals[i][index.1+1] != 0 { sum += vals[i][index.1+1]; done.1 = true; }
    }
    if !done.0 { sum += 1; }
    if !done.1 { sum += 1; }
    sum
}
