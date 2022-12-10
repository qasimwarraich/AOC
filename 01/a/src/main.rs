use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "./input.txt";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut max = 0;
    let mut counter = 0;
    for (_index, line) in reader.lines().enumerate() {
        let line = line?;
        println!("{:?} counter:{} max:{}", &line, counter, max);

        if &line == "" {
            if counter > max {
                max = counter;
            }
            counter = 0;
        } else {
            let count: i32 = line.parse()?;
            counter += count;
        }
    }

    println!("Highest Calories: {}", max);
    Ok(())
}
