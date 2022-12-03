use std::{collections::HashMap, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let file = include_str!("./input.txt");
    let file: String = file.into();

    let mut sum = 0;
    let alphabet: Vec<char> = ('a'..='z').chain('A'..='Z').collect();

    let elves: Vec<&str> = file.trim_end().split('\n').collect();
    let groups: Vec<&[&str]> = elves.chunks(3).collect();

    for group in groups {
        let mut compartment: HashMap<char, bool> = HashMap::new();
        let items1: Vec<char> = group[0].chars().collect();
        let items2: Vec<char> = group[1].chars().collect();
        let items3: Vec<char> = group[2].chars().collect();

        for char in items1 {
            compartment.insert(char, true);
        }

        for char in items2 {
            if compartment.contains_key(&char) {
                if items3.contains(&char) {
                    let value = alphabet
                        .to_owned()
                        .into_iter()
                        .position(|x| x == char)
                        .unwrap();
                    sum += value + 1;
                    break;
                }
            }
        }
    }
    println!("{:?}", sum);
    Ok(())
}
