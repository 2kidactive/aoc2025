use std::fs;
use std::iter::zip;

fn main() {
    let contents = fs::read_to_string("data.txt").expect("");

    let shapes: Vec<u64> = vec![7, 7, 5, 7, 7, 6];

    let input: Vec<&str> = contents
        .split("\n\n")
        .skip(6)
        .collect();

    let mut data: Vec<Vec<&str>> = input[0]
        .split("\n")
        .map(|x| x.split(" ").collect())
        .collect();

    data.pop();

    let mut sum = 0;
    for val in data {
        let area: u64 = val[0][0..val[0].len()-1]
            .split("x")
            .map(|x| x.parse::<u64>().unwrap())
            .product();

        let num_shapes: Vec<u64> = val[1..val.len()]
            .iter()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        let paired = zip(num_shapes.clone(), shapes.clone());
        let min_area: u64 = paired
            .map(|(a, b)| a * b)
            .sum();

        if area > min_area {
            sum += 1;
        }
    }
    println!("{sum}");
}
