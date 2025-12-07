use std::fs;

fn main() {
    let contents = fs::read_to_string("data.txt").expect("");
    let mut vals: Vec<Vec<char>> = Vec::new();
    let mut sum = 0;
    for (i, line) in contents.lines().enumerate() {
        let mut row: Vec<char> = Vec::new();
        for (j, c) in line.chars().enumerate() {
            row.push(c);
            match c {
                '^' => sum += check_splitter(vals.clone(), (i, j)),
                _ => continue,
            };
        }
        vals.push(row);
    }
    println!("{sum}");
}

fn check_splitter(vals: Vec<Vec<char>>, index: (usize, usize)) -> u32 {
    for i in (0..index.0).rev() {
        if vals[i][index.1] == '^' { return 0; }
        if vals[i][index.1 - 1] == '^' || vals[i][index.1] == 'S' || vals[i][index.1 + 1] == '^' { return 1; }
    }
    0
}
