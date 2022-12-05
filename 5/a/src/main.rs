use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file = include_str!("./input.txt");
    let file: String = file.into();

    let (stacks, instructions) = file.trim_end().split_once("\n\n").unwrap();

    let stacks: Vec<String> = stacks.split("\n").map(|x| x.to_string()).collect();
    let instructions: Vec<String> = instructions.split("\n").map(|x| x.to_string()).collect();

    let mut main_stack: Vec<Vec<String>> = Vec::new();

    let finder: Vec<String> = stacks[stacks.len() - 1]
        .split("")
        .map(|x| x.to_string())
        .collect();

    let cols = finder[finder.len() - 3].parse::<usize>()?;

    for _ in 0..cols {
        main_stack.push(vec![]);
    }

    for line in &stacks {
        let row_chars: Vec<char> = line.chars().collect();
        for (index, c) in row_chars.iter().enumerate() {
            if c.is_alphabetic() {
                let insert_index = finder[index + 1].parse::<usize>()? - 1;
                main_stack[insert_index].insert(0, c.to_string());
            }
        }
    }

    for instruction in instructions {
        let ivec: Vec<String> = instruction.split(" ").map(|x| x.to_string()).collect();

        let amount = ivec[1].parse::<i32>()?;
        let src = ivec[3].parse::<i32>()? - 1;
        let dst = ivec[5].parse::<i32>()? - 1;

        for _i in 0..amount {
            if let Some(elem) = main_stack[src as usize].pop() {
                main_stack[dst as usize].push(elem);
            }
        }
    }

    let mut result: String = String::new();
    for stack in main_stack {
        result.push_str(stack[stack.len() - 1].as_str());
    }

    println!("result = {:?}", result);

    Ok(())
}
