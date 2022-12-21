use std::{collections::HashMap, error::Error};

#[derive(Debug)]
struct Pair {
    lhs: String,
    rhs: String,
    op: String,
}

#[derive(Debug)]
struct Val {
    val: i64,
}

#[derive(Debug)]
enum Instruction {
    Pair(Pair),
    Val(Val),
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = include_str!("./input.test");
    // let file = include_str!("./input.txt");
    let file_vec: Vec<String> = file
        .trim_end()
        .split("\n")
        .map(|el| el.to_string())
        .collect();

    let mut monkey_numbers: HashMap<String, i64> = HashMap::new();
    let mut instruction_vec: Vec<Vec<String>> = Vec::new();
    let mut pairings: HashMap<String, Instruction> = HashMap::new();

    for row in &file_vec {
        let mut instruction: Vec<String> = row.split(" ").map(|el| el.to_string()).collect();
        instruction[0] = instruction[0].replace(":", "");

        if let Some(val) = instruction[1].parse::<i64>().ok() {
            monkey_numbers.insert(instruction[0].clone(), val);
            pairings.insert(instruction[0].clone(), Instruction::Val(Val { val }));
        } else {
            pairings.insert(
                instruction[0].clone(),
                Instruction::Pair(Pair {
                    lhs: instruction[1].clone(),
                    op: instruction[2].clone(),
                    rhs: instruction[3].clone(),
                }),
            );
        }

        instruction_vec.push(instruction);
    }

    let mut root_lhs = String::new();
    let mut root_rhs = String::new();
    for instruction in &mut instruction_vec {
        if instruction[0] == "root" {
            println!("instruction = {:?}", instruction);
            root_lhs = instruction[1].clone();
            root_rhs = instruction[3].clone();
        }
    }
    println!("root_lhs = {:?}", root_lhs);
    println!("root_rhs = {:?}", root_rhs);

    for pair in &pairings {
        println!("pair = {:?}", pair);
    }

    monkey_business(
        instruction_vec.clone(),
        monkey_numbers,
        &root_lhs,
        &root_rhs,
    );



    recurse_to_val(&pairings, &root_lhs);

    Ok(())
}

fn recurse_to_val(pairings: &HashMap<String, Instruction>, name: &String) {

    let spam = &pairings[name];

    println!("spam = {:?}", spam);
    match spam {
        Instruction::Val(_) => println!("It's a value!"),
        Instruction::Pair(pair) => recurse_to_val(pairings, &pair.lhs )
    }

    
}




fn monkey_business(
    instruction_vec: Vec<Vec<String>>,
    mut monkey_numbers: HashMap<String, i64>,
    root_lhs: &String,
    root_rhs: &String,
) {
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
    println!("monkey_numbers l = {:?}", monkey_numbers[root_lhs]);
    println!("monkey_numbers r = {:?}", monkey_numbers[root_rhs]);
}
