use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("data.txt").expect("");
    let points: Vec<Vec<u64>> = contents
        .lines()
        .map(|x| x
            .split(",")
            .map(|y| y.parse::<u64>().unwrap())
            .collect())
        .collect();

    // let dists: Vec<(u64, usize, usize)> = points.???():
    let mut dists: Vec<(u64, usize, usize)> = Vec::new();
    for (i, point) in points.iter().enumerate() {
        // let mut min = (u64::MAX, 0, 0);
        for (j, other) in points[i+1..].iter().enumerate() {
            let mut sum = 0;
            for k in 0..3 {
                sum += point[k].abs_diff(other[k]).pow(2);
            }
            // if sum < min.0 {
                // min = (sum, i, i+j+1);
            //}
            dists.push((sum, i, i+j+1));
        }
    }
    dists.sort();

    let boxes: Vec<Vec<usize>> = dists
        .iter()
        .map(|(_, x, y)| vec![*x, *y])
        .collect();

    let mut circuits = HashMap::new();

    let mut i = 0;
    let mut done = false;
    while !done {
        let a = circuits.get(&boxes[i][0]).copied().unwrap_or(0);
        let b = circuits.get(&boxes[i][1]).copied().unwrap_or(0);
        if a != 0 && b != 0 {
            if a == b { i += 1; continue; }
            for (key, val) in circuits.clone() {
                if val == b {
                    circuits.insert(key, a);
                }
            }
        } else if a != 0 {
            circuits.insert(boxes[i][1], a);
        } else if b != 0 {
            circuits.insert(boxes[i][0], b);
        } else {
            circuits.insert(boxes[i][0], i + 1);
            circuits.insert(boxes[i][1], i + 1);
        }
        let mut counts: Vec<(u64, usize)> = Vec::new();
        for (_, val) in circuits.clone() {
            let mut found = false;
            for i in 0..counts.len() {
                if val == counts[i].1 {
                    found = true;
                    counts[i].0 += 1;
                }
            }
            if !found {
                counts.push((1, val));
            }
        }
        if counts.len() == 1 && circuits.len() == 1000 {
            done = true;
            let x1 = points[boxes[i][0]][0];
            let x2 = points[boxes[i][1]][0];
            println!("{x1} {x2}");
        }
        i += 1;
    }
}
