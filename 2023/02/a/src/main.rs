use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<&str> = include_str!("./prod.input").trim().split("\n").collect();

    let mut sum = 0;
    let red_max = 12;
    let green_max = 13;
    let blue_max = 14;
    for line in input {
        let (id, rest) = line.split_once(": ").ok_or("failed to split on `:`")?;
        let id: i32 = id.replace("Game ", "").parse()?;

        let mut possible = true;
        'game: for game in rest.split("; ").map(|x| x.trim()) {
            for val in game.split(", ") {
                let tokens: Vec<&str> = val.split(" ").collect();
                let qty: i32 = tokens[0].trim().parse()?;
                match tokens[1] {
                    "red" => possible = qty <= red_max,
                    "green" => possible = qty <= green_max,
                    "blue" => possible = qty <= blue_max,
                    _ => println!("wut"),
                }
                if !possible {
                    break 'game;
                }
            }
        }
        if possible {
            sum += id;
        }
    }
    println!("{}", sum);
    Ok(())
}
