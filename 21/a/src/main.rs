use std::{collections::HashMap, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    // let file = include_str!("./input.test");
    let file = include_str!("./input.txt");
    let file_vec: Vec<String> = file
        .trim_end()
        .split("\n")
        .map(|el| el.to_string())
        .collect();

    let mut monkey_numbers: HashMap<String, i64> = HashMap::new();
    let mut instruction_vec: Vec<Vec<String>> = Vec::new();

    for row in &file_vec {
        let instruction: Vec<String> = row.split(" ").map(|el| el.to_string()).collect();
        let name = instruction[0].replace(":", "");

        if let Some(val) = instruction[1].parse::<i64>().ok() {
            monkey_numbers.insert(name, val);
        }
        instruction_vec.push(instruction);
    }

    for instruction in &mut instruction_vec {
        instruction[0] = instruction[0].replace(":", "");
        if instruction[0] == "root" {
            println!("instruction = {:?}", instruction);
        }
    }

    monkey_business(instruction_vec, monkey_numbers);


    Ok(())
}

fn monkey_business(instruction_vec: Vec<Vec<String>>, mut monkey_numbers: HashMap<String, i64>) {
    while monkey_numbers.len() < instruction_vec.len() {
        for instruction in &instruction_vec {
            let name = instruction[0].clone();

            if !monkey_numbers.contains_key(&name) {
                if monkey_numbers.contains_key(&instruction[1])
                    && monkey_numbers.contains_key(&instruction[3])
                {
                    let lhs = monkey_numbers.get(&instruction[1]).unwrap();
                    let rhs = monkey_numbers.get(&instruction[3]).unwrap();
                    let res: i64;
                    match instruction[2].as_str() {
                        "-" => {
                            res = lhs - rhs;
                            monkey_numbers.insert(name, res);
                        }
                        "+" => {
                            res = lhs + rhs;
                            monkey_numbers.insert(name, res);
                        }
                        "/" => {
                            res = lhs / rhs;
                            monkey_numbers.insert(name, res);
                        }
                        "*" => {
                            res = lhs * rhs;
                            monkey_numbers.insert(name, res);
                        }
                        _ => println!("Something went wrong"),
                    }
                }
            }
        }
    }
    println!("monkey_numbers = {:?}", monkey_numbers);

    let res = monkey_numbers.get("root").unwrap();
    println!("res = {:?}", res);
}
