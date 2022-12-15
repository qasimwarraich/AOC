use std::error::Error;

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn check_abyss(&self, map: &Vec<Vec<char>>) -> bool {
        for i in self.y..map.len() {
            if map[i][self.x] == '#' || map[i][self.x] == 'o' {
                return false;
            }
        }
        return true;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // let file = include_str!("./input.test");
    let file = include_str!("./input.txt");
    let (rocks, mut map, max_y, start) = parse_file(file);
    let (_rocks, map) = draw_rocks(rocks, &mut map);
    let start_pos: Pos = Pos { x: start, y: 0 };
    let mut drop_count = 0;
    let mut abyss = false;
    println!("max_y = {:?}", max_y);
    println!("start_pos = {:?}", start_pos);
    while !abyss {
        drop_count += 1;
        let mut drop = start_pos;
        loop {
            if drop.check_abyss(map) {
                abyss = true;
                println!("abyss  = {:?}", drop);
                break;
            }
            if map[drop.y + 1][drop.x] == ' ' {
                drop.y += 1;
            } else if map[drop.y + 1][drop.x] == 'o' || map[drop.y + 1][drop.x] == '#' {
                if map[drop.y + 1][drop.x - 1] == ' ' {
                    drop.y += 1;
                    drop.x -= 1;
                } else if map[drop.y + 1][drop.x + 1] == ' ' {
                    drop.y += 1;
                    drop.x += 1;
                } else {
                    map[drop.y][drop.x] = 'o';
                    break;
                }
            } else if map[drop.y + 1][drop.x] == 'o' {
                continue;
            } else {
                map[drop.y][drop.x] = 'o';
                break;
            }
        }
        // print_map(map);
        pretty_print_map(map);
        println!("drop_count = {:?}", drop_count);
    }

    Ok(())
}

fn draw_rocks(
    rocks: Vec<Vec<Pos>>,
    map: &mut Vec<Vec<char>>,
) -> (Vec<Vec<Pos>>, &mut Vec<Vec<char>>) {
    for formation in &rocks {
        let formations: Vec<_> = formation.windows(2).collect();

        for range in formations {
            let x_1 = range[0].x;
            let y_1 = range[0].y;
            let x_2 = range[1].x;
            let y_2 = range[1].y;

            if x_1 == x_2 {
                let y_range = y_1..=y_2;
                for i in y_range {
                    map[i][x_1] = '#';
                }
            }
            if y_1 == y_2 {
                let x_range = x_2..=x_1;
                for i in x_range {
                    map[y_1][i] = '#';
                }
            }
        }
    }
    return (rocks, map);
}

fn parse_file(file: &str) -> (Vec<Vec<Pos>>, Vec<Vec<char>>, usize, usize) {
    let file_vec: Vec<_> = file.trim_end().split("\n").collect();
    let mut rock_vec: Vec<Vec<Pos>> = Vec::new();

    for row in file_vec {
        let mut rock_formation: Vec<Pos> = Vec::new();
        let coordinates: Vec<_> = row.split(" -> ").collect();
        for coordinate in coordinates {
            let positions: Vec<usize> = coordinate
                .split(",")
                .filter_map(|el| el.parse().ok())
                .collect();
            let pos = Pos {
                x: positions[0],
                y: positions[1],
            };
            rock_formation.push(pos);
        }
        rock_vec.push(rock_formation);
    }

    let mut min_x: usize = 500;
    let mut max_x: usize = 1;
    let mut min_y: usize = 500;
    let mut max_y: usize = 1;
    for row in &rock_vec {
        for pos in row {
            if pos.x > max_x {
                max_x = pos.x;
            } else if pos.x < min_x {
                min_x = pos.x;
            }
            if pos.y > max_y {
                max_y = pos.y;
            } else if pos.y < min_y {
                min_y = pos.y;
            }
        }
    }
    println!("min_x = {:?}", min_x);
    println!("max_x = {:?}", max_x);
    println!("min_y = {:?}", min_y);
    println!("max_y = {:?}", max_y);

    let xrange = max_x - min_x;
    let yrange = max_y - min_y;
    let start = 500 - min_x + 1;

    println!("xrange = {:?}", xrange);
    println!("yrange = {:?}", yrange);
    println!("start = {:?}", start);

    for row in &mut rock_vec {
        for pos in row {
            pos.x = pos.x - min_x + 1;
            pos.y = pos.y - min_y + 4;
        }
    }

    let grid: Vec<Vec<char>> = vec![vec![' '; xrange + 2]; yrange + 10];

    println!("grid.len() = {:?}", grid.len());
    println!("grid[0].len() = {:?}", grid.len());

    return (rock_vec, grid, max_y, start);
}

fn print_map(map: &Vec<Vec<char>>) {
    print!("");
    for j in 0..11 {
        print!("  {}  ", j);
    }
    println!("");

    for (i, row) in map.iter().enumerate() {
        println!("{:?}", row);
    }
}

fn pretty_print_map(map: &Vec<Vec<char>>) {
    for row in map.iter() {
        let spam: Vec<String> = row.iter().map(|el| el.to_string()).collect();
        let spam = spam.join("");
        println!("spam = {:?}", spam);
    }
}
