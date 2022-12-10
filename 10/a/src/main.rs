use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file = include_str!("./input.txt");

    let file_vec: Vec<String> = file
        .trim_end()
        .split("\n")
        .map(|el| el.to_string())
        .collect();

    let cycles: Vec<i32> = vec![20, 60, 100, 140, 180, 220];
    let mut results: Vec<i32> = Vec::new();
    let mut x_reg = 1;
    let mut cycle = 0;

    for command in file_vec {
        if command.starts_with("addx") {
            let command: Vec<_> = command.split(" ").collect();
            let num: i32 = command[1].parse()?;

            for _i in 0..2 {
                cycle += 1;
                if cycles.contains(&cycle) {
                    results.push(cycle * x_reg)
                }
            }
            x_reg += num
        } else {
            cycle += 1;
            if cycles.contains(&cycle) {
                results.push(cycle * x_reg)
            }
        }
    }

    let res: i32 = results.iter().sum();
    println!("res = {:?}", res);

    Ok(())
}
