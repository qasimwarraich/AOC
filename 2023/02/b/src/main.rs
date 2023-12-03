use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<&str> = include_str!("./prod.input").trim().split("\n").collect();

    let mut sum = 0;
    for line in input {
        let (_, rest) = line.split_once(": ").ok_or("Failed to split once on `:`")?;
        let mut red_max = 0;
        let mut green_max = 0;
        let mut blue_max = 0;
        for game in rest.split("; ") {
            for vals in game.split(", ") {
                let tokens: Vec<&str> = vals.split(" ").collect();
                let qty: i32 = tokens[0].parse()?;
                match tokens[1] {
                    "red" => {
                        if qty > red_max {
                            red_max = qty
                        }
                    }
                    "green" => {
                        if qty > green_max {
                            green_max = qty
                        }
                    }
                    "blue" => {
                        if qty > blue_max {
                            blue_max = qty
                        }
                    }
                    _ => println!("wut"),
                }
            }
        }
        sum += red_max * green_max * blue_max
    }
    println!("{}", sum);
    Ok(())
}
