use std::fs;

fn main() {
    let contents = fs::read_to_string("data.txt").expect("");
    let mut map: Vec<u8> = Vec::new();
    let mut len = 0;
    let mut sum = 0;
    for row in contents.lines() {
        len = row.len();
        for roll in row.chars() {
            let val = match roll {
                '.' => 0,
                '@' => 1,
                _ => 2
            };
            map.push(val);
        }
    }
    let mut done = false;
    while !done {
        done = true;
        let mut changes: Vec<usize> = Vec::new();
        for (i, e) in map.iter().enumerate() {
            if *e == 1 {
                let mut j = 0;
                if i % len != 0 {
                    if i >= len && map[i-len-1] == 1 {
                        j += 1;
                    }
                    if i < map.len() - len && map[i+len-1] == 1 {
                        j += 1;
                    }
                    if map[i-1] == 1 {
                        j += 1;
                    }
                }
                if i % len != len-1 {
                    if i >= len && map[i-len+1] == 1 {
                        j += 1;
                    }
                    if i < map.len() - len && map[i+len+1] == 1 {
                        j += 1;
                    }
                    if map[i+1] == 1 {
                        j += 1;
                    }
                }
                if i >= len && map[i-len] == 1 {
                    j += 1;
                }
                if i < map.len() - len && map[i+len] == 1 {
                    j += 1;
                }
                if j < 4 {
                    done = false;
                    changes.push(i);
                    sum += 1;
                }
            }
        }
        for n in changes {
            map[n] = 0;
        }
    }
    println!("{sum}");
}
