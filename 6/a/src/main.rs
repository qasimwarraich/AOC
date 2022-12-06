use std::{collections::HashSet, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let file = include_str!("./input.txt");
    let file_vec: Vec<String> = file.trim_end().split("").map(|x| x.to_string()).collect();
    let message_len = 4;

    let windows = file_vec[1..].windows(message_len);

    for (index, window) in windows.enumerate() {
        let mut set = HashSet::new();
        if window.iter().all(|el| set.insert(el)) {
            println!("res = {:?}", index + message_len);
            break;
        }
    }
    Ok(())
}
