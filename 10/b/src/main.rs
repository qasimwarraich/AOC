use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file = include_str!("./input.txt");

    let file_vec: Vec<String> = file
        .trim_end()
        .split("\n")
        .map(|el| el.to_string())
        .collect();

    let mut x_reg = 1;
    let mut _cycle = 0;
    let mut pos = 0;

    for command in file_vec {
        if command.starts_with("addx") {
            let command: Vec<_> = command.split(" ").collect();
            let num: i32 = command[1].parse()?;

            for _i in 0..2 {
                _cycle += 1;
                if pos == 39 {
                    pos = 0;
                    println!("");
                    continue;
                }
                if ((x_reg - 1)..=(x_reg + 1)).contains(&pos) {
                    print!("#")
                } else {
                    print!(" ")
                }
                pos += 1
            }
            x_reg += num
        } else {
            _cycle += 1;
            if pos == 39 {
                pos = 0;
                println!("");
                continue;
            }
            if ((x_reg - 1)..=(x_reg + 1)).contains(&pos) {
                print!("#")
            } else {
                print!(" ")
            }
            pos += 1
        }
    }

    Ok(())
}
