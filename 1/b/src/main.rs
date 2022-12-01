use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> Result<(), io::Error> {
    let filename = "./input.txt";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut counter = 0;
    let mut elves: Vec<i32> = Vec::new();

    for (_index, line) in reader.lines().enumerate() {
        let line = line?;

        if &line == "" {
            elves.push(counter);
            counter = 0;
        } else {
            let count: i32 = line.parse().unwrap();
            counter += count;
        }
    }

    elves.sort();

    let top3 = &elves[elves.len() - 3..];
    let sum: i32 = top3.iter().sum();
    println!("Top3 Sum: {:?}", sum);

    Ok(())
}
