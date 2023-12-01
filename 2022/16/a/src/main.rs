use std::{collections::HashMap, error::Error};

#[derive(Hash, Debug, PartialEq, Eq)]
struct Valve {
    name: String,
    flow_rate: usize,
    neighbours: Vec<Valve>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = include_str!("./input.test");
    parse_file(file);

    Ok(())
}

fn parse_file(file: &str) {
    let mut valve_vec: Vec<Valve> = Vec::new();

    let file_vec: Vec<String> = file
        .trim_end()
        .split("\n")
        .map(|el| el.to_string())
        .collect();

    println!("file_vec = {:?}", file_vec);

    let mut neighbours: HashMap<String, Vec<String>> = HashMap::new();

    for row in file_vec {
        let row_vec: Vec<String> = row.trim_end().split(" ").map(|el| el.to_string()).collect();

        let name = row_vec[1].clone();

        let flow_rate: Vec<_> = row_vec[4].split("=").collect();
        let flow_rate: String = flow_rate[1].chars().filter(|el| el != &';').collect();
        let flow_rate: usize = flow_rate.parse::<usize>().ok().unwrap();

        let valve = Valve {
            name,
            flow_rate,
            neighbours: vec![],
        };

        let neighbour_slice = &row_vec[9..];
        let neighbour_vec: Vec<_> = neighbour_slice
            .iter()
            .map(|el| el.replace(",", ""))
            .collect();

            neighbours.insert(valve.name.clone(), neighbour_vec.clone());

        valve_vec.push(valve);
    }

    let fill_list: Vec<Vec<Valve>> = Vec::new();
    for valve in &valve_vec {
        let fill: Vec<<Valve>> = Vec::new();
        let valve_neighbours = neighbours.get(&valve.name).unwrap();
        for neighbour in valve_neighbours  {
            let neighbour_valve = valve_vec.iter().find(|el| el.name == *neighbour).unwrap();
            fill.push(neighbour_valve);

            println!("neighbour_valve = {:?}", neighbour_valve);
        }
    }
}
