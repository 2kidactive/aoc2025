use std::fs;
use good_lp::*;

fn main() {
    let contents = fs::read_to_string("../../data.txt").expect("");
    let machines: Vec<Vec<String>> = contents
        .lines()
        .map(|x| x
            .split(" ")
            .map(|x| String::from(x))
            .collect())
        .collect();

    let mut buttons: Vec<Vec<Vec<u16>>> = Vec::new();
    let mut totals: Vec<Vec<u16>> = Vec::new();
    for machine in machines {
        let mut button_row: Vec<Vec<u16>> = Vec::new();
        for i in 1..machine.len()-1 {
            button_row.push(machine[i]
                .trim_matches(&['(', ')'])
                .split(',')
                .map(|x| x.parse::<u16>().unwrap())
                .collect());
        }
        buttons.push(button_row);
        totals.push(machine[machine.len()-1]
            .trim_matches(&['{', '}'])
            .split(',')
            .map(|x| x.parse::<u16>().unwrap())
            .collect());
    }

    let mut sum: u32 = 0;

    for i in 0..buttons.len() {
        let mut problem = variables!();
        let vars = vec![variable().integer().min(0); buttons[i].len()];
        let x: Vec<Variable> = problem.add_all(vars);
        let objective: Expression = x.iter().sum();
        let mut model = problem.minimise(objective).using(default_solver);
        for j in 0..totals[i].len() {
            let mut z: Vec<Variable> = Vec::new();

            for (k, a) in buttons[i].iter().enumerate() {
                if a.contains(&(j as u16)) {
                    z.push(x[k]);
                }
            }

            model = model.with(z.iter().sum::<Expression>().eq(totals[i][j]));
        }
        let solution = model.solve().unwrap();
        for i in x {
            sum += solution.value(i).round() as u32;
        }
    }
    println!("{sum}");
}
