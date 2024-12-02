use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<&str> = include_str!("./prod.input").trim().split("\n").collect();
    let mut total = 0;

    for line in input {
        let mut points = 0;
        let (_id, rest) = line.split_once(": ").ok_or("failed to split on `:`")?;
        let (winning, actual) = rest.split_once("|").ok_or("failed to split on `|`")?;
        let winning: Vec<&str> = winning.trim().split(" ").filter(|&s| s != "").collect();
        let actual: Vec<&str> = actual.trim().split(" ").filter(|&s| s != "").collect();
        println!("actual = {:?}", actual);

        for num in actual {
            if winning.contains(&num) {
                if points == 0 {
                    points = 1;
                } else {
                    points = points * 2;
                }
            }
        }
        total += points
    }
    println!("{}", total);
    Ok(())
}
