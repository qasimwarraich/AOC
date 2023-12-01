use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file = include_str!("./input.txt");
    let file: String = file.into();

    let (stacks, instructions) = parse_file(file);
    let column_finder = create_finder(&stacks);
    let mut main_stack = create_main_stack(&column_finder)?;

    init_stacks(stacks, &mut main_stack, &column_finder)?;
    move_crates(instructions, &mut main_stack);

    let result = assemble_result(&mut main_stack);

    println!("result = {:?}", result);

    Ok(())
}

fn move_crates(instructions: Vec<String>, main_stack: &mut Vec<Vec<String>>) {
    for instruction in instructions {
        let ivec: Vec<usize> = instruction
            .split_whitespace()
            .filter_map(|token| token.parse().ok())
            .collect();

        let amount = ivec[0];
        let src = ivec[1] - 1;
        let dst = ivec[2] - 1;

        for _i in 0..amount {
            if let Some(elem) = main_stack[src].pop() {
                main_stack[dst].push(elem);
            }
        }
    }
}

fn init_stacks(
    stacks: Vec<String>,
    main_stack: &mut Vec<Vec<String>>,
    column_finder: &Vec<String>,
) -> Result<(), Box<dyn Error>> {
    for line in &stacks {
        let row_chars: Vec<char> = line.chars().collect();
        for (index, c) in row_chars.iter().enumerate() {
            if c.is_alphabetic() {
                let insert_index = column_finder[index + 1].parse::<usize>()? - 1;
                main_stack[insert_index].insert(0, c.to_string());
            }
        }
    }
    Ok(())
}

fn create_main_stack(column_finder: &Vec<String>) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let mut main_stack: Vec<Vec<String>> = Vec::new();
    let num_cols = column_finder[column_finder.len() - 3].parse::<usize>()?;

    for _ in 0..num_cols {
        main_stack.push(vec![]);
    }

    Ok(main_stack)
}

fn create_finder(stacks: &Vec<String>) -> Vec<String> {
    let column_finder: Vec<String> = stacks[stacks.len() - 1]
        .split("")
        .map(|x| x.to_string())
        .collect();
    return column_finder;
}

fn assemble_result(main_stack: &mut Vec<Vec<String>>) -> String {
    let mut result: String = String::new();
    for stack in main_stack {
        result.push_str(stack[stack.len() - 1].as_str());
    }
    return result;
}

fn parse_file(file: String) -> (Vec<String>, Vec<String>) {
    let (stacks, instructions) = file.trim_end().split_once("\n\n").unwrap();

    let stacks: Vec<String> = stacks.split("\n").map(|x| x.to_string()).collect();
    let instructions: Vec<String> = instructions.split("\n").map(|x| x.to_string()).collect();
    return (stacks, instructions);
}
