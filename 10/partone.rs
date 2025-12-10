use std::fs;
use std::collections::VecDeque;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("data.txt").expect("");
    let machines: Vec<Vec<String>> = contents
        .lines()
        .map(|x| x
            .split(" ")
            .map(|x| String::from(x))
            .collect())
        .collect();

    let mut lights: Vec<u32> = Vec::new();
    let mut buttons: Vec<Vec<u32>> = Vec::new();
    for machine in machines {
        let light: u32 = machine[0]
            .trim_matches(&['[', ']'])
            .chars()
            .enumerate()
            .fold(0, |mut acc, (i, x)| {
                match x {
                    '#' => acc += 2_u32.pow(i as u32),
                    _ => (),
                };
                acc
            });
        lights.push(light);

        let mut button_row: Vec<u32> = Vec::new();
        for i in 1..machine.len()-1 {
            button_row.push(machine[i]
                .trim_matches(&['(', ')'])
                .split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .fold(0, |mut acc, x| {
                    acc += 2_u32.pow(x);
                    acc
                }));
        }
        buttons.push(button_row);
    }
    let mut sum = 0;
    for i in 0..lights.len() {
        sum += bfs(lights[i], buttons[i].clone());
    }
    println!("{sum}");
}

fn bfs(lights: u32, buttons: Vec<u32>) -> u32 {
    let mut queue: VecDeque<(u32, u32)> = VecDeque::new();
    let mut seen: HashSet<u32> = HashSet::new();
    seen.insert(0);
    queue.push_back((0, 0));
    while queue.len() != 0 {
        let (v, current_depth) = queue.pop_front().unwrap();
        if v ^ lights == 0 {
            return current_depth;
        }
        for i in &buttons {
            let next = v ^ i;
            if !seen.contains(&next) {
                seen.insert(next);
                queue.push_back((next, current_depth+1));
            }
        }
    }
    0
}
