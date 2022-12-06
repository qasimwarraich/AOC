use std::{collections::HashSet, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let file = include_str!("./input.txt");
    let file: String = file.into();
    let file: Vec<String> = file.trim_end().split("\n").map(|x| x.to_string()).collect();
    let message_len = 14;

    for string in file {
        let svec = string
            .split("")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        let sl1 = &svec[1..];
        let windows = sl1.windows(message_len);

        for window in windows {
            let set = window.iter().collect::<HashSet<_>>();
            if set.len() == message_len {
                let search = window.concat();
                let index = string.find(search.as_str());
                let res = index.unwrap() + message_len;
                println!("res = {:?}", res);
                break;
            }
        }
    }
    Ok(())
}
