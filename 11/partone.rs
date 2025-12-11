use std::fs;
use std::collections::HashMap;

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

    let mut stack: Vec<String> = Vec::new();
    stack.push(String::from("you"));

    let mut sum = 0;
    while stack.len() > 0 {
        let v: String = stack.pop().expect("");
        if v == "out" {
            sum += 1;
            continue;
        }
        let next: Vec<String> = device_map.get(&v).unwrap().to_vec();
        for n in next { 
            stack.push(n);
        }
    }
    println!("{sum}");
}
