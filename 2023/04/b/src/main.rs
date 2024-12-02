use std::{collections::HashMap, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<&str> = include_str!("./prod.input").trim().split("\n").collect();
    let mut card_totals: Vec<i32> = vec![1; input.len()];
    let mut points_map = HashMap::new();
    card_totals[0] = 1;

    for (index, &line) in input.iter().enumerate() {
        let mut points = 0;
        let (id, rest) = line.split_once(": ").ok_or("failed to split on `:`")?;
        let id = id
            .trim()
            .split(" ")
            .last()
            .ok_or("fialed to get id")?
            .parse::<usize>()?;
        let (winning, actual) = rest.split_once("|").ok_or("failed to split on `|`")?;
        let winning: Vec<&str> = winning.trim().split(" ").filter(|&s| s != "").collect();
        let actual: Vec<&str> = actual.trim().split(" ").filter(|&s| s != "").collect();

        println!("actual = {:?}", actual);
        for num in actual {
            if winning.contains(&num) {
                points += 1
            }
        }
        points_map.insert(id as usize, points as usize);
        println!("card_map = {:?}", points_map);
    }
    for i in 1..6 {
        println!("i = {:?}", i);
        let spam = points_map[&i];
        // let points = points_map.get(&i).ok_or("map fail")?;

        for x in 0..card_totals[i] {
            for j in 1..spam + 1 {
                card_totals[i + j] += 1;
            }
        }
    }
    println!("card_map = {:?}", points_map);
    println!("card_totals = {:?}", card_totals);

    let sum = card_totals.iter().fold(0, |a, &b| a + b);
    println!("sum = {:?}", sum);
    Ok(())
}
