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

    let game_code = HashMap::from([("X", 1), ("Y", 2), ("Z", 3), ("A", 1), ("B", 2), ("C", 3)]);
    let moves = Vec::from([1, 2, 3]);
    let mut total_score = 0;

    let lines = reader.lines().enumerate();
    for (_index, line) in lines {
        let line = line?;
        let char_vec = &line.split(" ").collect::<Vec<&str>>();
        let p1 = game_code[char_vec[0]];
        let p2 = game_code[char_vec[1]];

        if p1 == p2 {
            total_score += 3 + p2;
        } else if moves[p1 % 3] == p2 {
            total_score += 6 + p2;
        } else {
            total_score += p2;
        }
    }
    println!("{}", total_score);
    Ok(())
}
