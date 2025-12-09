use std::fs;

fn main() {
    let contents = fs::read_to_string("data.txt").expect("");
    let points: Vec<Vec<u64>> = contents
        .lines()
        .map(|x| x.split(",").map(|y| y.parse::<u64>().unwrap()).collect())
        .collect();

    let areas: Vec<u64> = points
        .iter()
        .enumerate()
        .flat_map(|(i, point)| {
            points[i + 1..]
                .iter()
                .map(|other| (point[0].abs_diff(other[0]) + 1) * (point[1].abs_diff(other[1]) + 1))
                .collect::<Vec<u64>>()
        })
        .collect();

    println!("{}", areas.iter().max().expect(""));
}
