use std::collections::VecDeque;
use std::error::Error;

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Op,
    test: u64,
    t_move: usize,
    f_move: usize,
}

#[derive(Debug)]
struct Op {
    op: String,
    value: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = include_str!("./input.txt");

    let file_vec = parse_file(file);

    let mut inspections: Vec<u64> = vec![0, 0, 0, 0, 0, 0, 0, 0];

    let mut monkeys = parse_monkey(file_vec)?;

    for _i in 0..10000 {
        for i in 0..monkeys.len() {
            for item in monkeys[i].items.clone() {
                inspections[i] += 1;
                monkeys[i].items.pop_front().unwrap();

                let value;

                if monkeys[i].operation.value == "old" {
                    value = item;
                } else {
                    value = monkeys[i].operation.value.parse()?;
                }

                let mut new_value = match monkeys[i].operation.op.as_str() {
                    "*" => item * value,
                    "+" => item + value,
                    _ => item,
                };

                let common_multiple: u64 = monkeys.iter().map(|monkey| monkey.test).product();
                let test = new_value % monkeys[i].test == 0;

                new_value = new_value % common_multiple;

                if test {
                    let index = monkeys[i].t_move;
                    monkeys[index].items.push_back(new_value);
                } else {
                    let index = monkeys[i].f_move;
                    monkeys[index].items.push_back(new_value);
                }
            }
        }
    }
    inspections.sort();
    let res = inspections[inspections.len() - 2] * inspections[inspections.len() - 1];
    println!("res = {:?}", res);

    Ok(())
}

fn parse_monkey(monkey_vec: Vec<Vec<String>>) -> Result<Vec<Monkey>, Box<dyn Error>> {
    let mut monkeys: Vec<Monkey> = Vec::new();

    for monkey in monkey_vec {
        let mut items_vec: VecDeque<u64> = VecDeque::new();
        let items_raw: Vec<_> = monkey[1].split([' ', ',']).filter(|el| el != &"").collect();
        for item in items_raw[2..].iter() {
            items_vec.push_back(item.parse().unwrap());
        }

        let operation_raw: Vec<String> = monkey[2]
            .split_whitespace()
            .map(|el| el.to_string())
            .collect();
        let op: Op = Op {
            op: operation_raw[4].to_owned(),
            value: operation_raw[5].to_owned(),
        };

        let test: u64 = monkey[3]
            .split_whitespace()
            .into_iter()
            .nth(3)
            .unwrap()
            .parse()?;
        let t_move: usize = monkey[4]
            .split_whitespace()
            .into_iter()
            .nth(5)
            .unwrap()
            .parse()?;
        let f_move: usize = monkey[5]
            .split_whitespace()
            .into_iter()
            .nth(5)
            .unwrap()
            .parse()?;

        monkeys.push(Monkey {
            items: items_vec,
            operation: op,
            test: test,
            t_move: t_move,
            f_move: f_move,
        })
    }

    Ok(monkeys)
}

fn parse_file(file: &str) -> Vec<Vec<String>> {
    let file_vec: Vec<String> = file
        .trim_end()
        .split("\n\n")
        .map(|el| el.to_string())
        .collect();

    let mut monkey_vec: Vec<Vec<String>> = Vec::new();

    for entry in file_vec.as_slice() {
        monkey_vec.push(
            entry
                .trim_end()
                .split("\n")
                .map(|el| el.trim().to_string())
                .collect(),
        );
    }

    return monkey_vec;
}
