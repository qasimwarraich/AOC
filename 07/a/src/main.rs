use std::{collections::HashMap, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let file = include_str!("./input.txt");

    let mut log_vec: Vec<String> = file.trim_end().split("$").map(|x| x.to_string()).collect();

    let mut sum = 0;
    log_vec = log_vec
        .iter()
        .map(|x| x.replace("\n", " "))
        .map(|x| x.trim().to_string())
        .filter(|x| x != "")
        .collect();

    let mut file_tree: Vec<String> = Vec::new();
    let mut dir_sizes: HashMap<String, usize> = HashMap::new();
    let mut cwd: String;

    file_tree.push("/".to_string());

    for entry in log_vec {
        if entry.starts_with("cd") {
            let c: Vec<_> = entry.split(" ").collect();
            if c[1] == ".." {
                file_tree.pop();
                continue;
            }
            cwd = c[1].to_string();
            if file_tree.last().unwrap() != &cwd {
                file_tree.push(cwd.clone());
            }
        }

        let file_sizes: Vec<usize> = entry
            .split_whitespace()
            .filter_map(|token| token.parse().ok())
            .collect();

        let mut path = "".to_string();
        for file in &file_tree {
            let dir_name = format!("{}{}", file, "/");
            path += &dir_name;

            for file_size in &file_sizes {
                if let Some(count) = dir_sizes.get(&path) {
                    dir_sizes.insert(path.clone(), count + file_size);
                } else {
                    dir_sizes.insert(path.clone(), *file_size);
                }
            }
        }
    }

    for (_entry, value) in &dir_sizes {
        if *value <= 100_000 {
            sum += value;
        }
    }

    println!("sum = {:?}", sum);
    Ok(())
}
