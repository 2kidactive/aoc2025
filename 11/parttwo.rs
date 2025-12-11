use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("data.txt").expect("");
    let devices: Vec<Vec<String>> = contents
        .lines()
        .map(|x| x
            .split(" ")
            .map(|x| String::from(x.trim_matches(':')))
            .collect())
        .collect();

    let mut device_map: HashMap<String, Vec<String>> = HashMap::new();
    for device in devices {
        let mut outputs: Vec<String> = Vec::new();
        for i in 1..device.len() {
            outputs.push(device[i].clone());
        }
        device_map.insert(device[0].clone(), outputs);
    }

    let mut cache: HashMap<String, (u64, u64, u64, u64)> = HashMap::new();
    let sum = dfs(&device_map, String::from("svr"), HashSet::new(), &mut cache);
    dbg!(sum);
}

fn dfs(graph: &HashMap<String, Vec<String>>, v: String, mut seen: HashSet<String>, cache: &mut HashMap<String, (u64, u64, u64, u64)>) -> (u64, u64, u64, u64) {
    if v == "out" {
        return (1, 0, 0, 0);
    } else if seen.contains(&v) {
        return (0, 0, 0, 0);
    }
    match cache.get(&v) {
        Some(x) => { return *x; },
        None => {},
    };

    seen.insert(v.clone());
    let next: Vec<String> = graph.get(&v).unwrap().to_vec();
    let mut sum = (0, 0, 0, 0);
    for n in next {
        let val = dfs(graph, n.clone(), seen.clone(), cache);
        cache.insert(n.clone(), val);
        sum.0 += val.0;
        sum.1 += val.1;
        sum.2 += val.2;
        sum.3 += val.3;
    }
    if v == "fft" {
        sum.1 = sum.0;
        sum.0 = 0;

        sum.3 = sum.2;
        sum.2 = 0;
    } else if v == "dac" {
        sum.2 = sum.0;
        sum.0 = 0;

        sum.3 = sum.1;
        sum.1 = 0;
    }
    return sum;
}
