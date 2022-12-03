use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "./input.txt";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut duplicates: Vec<char> = Vec::new();
    let alphabet: Vec<char> = ('a'..='z').chain('A'..='Z').collect();

    for line in reader.lines(){
        let line = line?;
        let half = line.len() / 2;

        let line: Vec<char> = line.chars().collect();
        let sl1 = &line[0..half];
        let sl2 = &line[half..];

        let mut compartment1: HashMap<&char, bool> = HashMap::new();
        for item in sl1 {
            compartment1.insert(item, true);
        }
        for item in sl2 {
            if compartment1.contains_key(item) {
                duplicates.push(item.to_owned());
                break;
            }
        }
    }

    let mut sum = 0;
    for item in duplicates {
        let value = alphabet
            .to_owned()
            .into_iter()
            .position(|x| x == item)
            .unwrap();
        sum += value + 1;
    }

    println!("{:?}", sum);
    Ok(())
}
