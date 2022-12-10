use std::collections::HashSet;

#[derive(Debug)]
struct Move {
    direction: String,
    steps: i32,
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

fn main() {
    let file = include_str!("./input.txt");
    let file_vec: Vec<_> = file.trim_end().split("\n").collect();
    let mut move_vec: Vec<Move> = Vec::new();
    let mut move_set: HashSet<Pos> = HashSet::new();

    for pair in &file_vec {
        let direction: Vec<String> = pair.split_whitespace().map(|x| x.to_string()).collect();

        let spam = Move {
            direction: direction[0].clone(),
            steps: direction[1].parse().unwrap(),
        };
        move_vec.push(spam);
    }

    let mut rope = vec![
        Pos { x: 0, y: 0 },
        Pos { x: 0, y: 0 },
        Pos { x: 0, y: 0 },
        Pos { x: 0, y: 0 },
        Pos { x: 0, y: 0 },
        Pos { x: 0, y: 0 },
        Pos { x: 0, y: 0 },
        Pos { x: 0, y: 0 },
        Pos { x: 0, y: 0 },
        Pos { x: 0, y: 0 },
    ];

    for step in &move_vec {
        for _i in 0..step.steps {
            if step.direction == "L" {
                rope[0].x -= 1;
            }
            if step.direction == "R" {
                rope[0].x += 1;
            }
            if step.direction == "U" {
                rope[0].y += 1;
            }
            if step.direction == "D" {
                rope[0].y -= 1;
            }

            for k in 0..9 {
                let d_x = rope[k].x.abs_diff(rope[k + 1].x);
                let d_y = rope[k].y.abs_diff(rope[k + 1].y);

                if d_x > 1 || d_y > 1 {
                    if rope[k + 1].x != rope[k].x && rope[k + 1].y != rope[k].y {
                        if rope[k].x > rope[k + 1].x && rope[k].y > rope[k + 1].y {
                            rope[k + 1].x += 1;
                            rope[k + 1].y += 1;
                        } else if rope[k].x < rope[k + 1].x && rope[k].y < rope[k + 1].y {
                            rope[k + 1].x -= 1;
                            rope[k + 1].y -= 1;
                        } else if rope[k].x > rope[k + 1].x && rope[k].y < rope[k + 1].y {
                            rope[k + 1].x += 1;
                            rope[k + 1].y -= 1;
                        } else {
                            rope[k + 1].x -= 1;
                            rope[k + 1].y += 1;
                        }
                    } else {
                        if rope[k].y > rope[k + 1].y {
                            rope[k + 1].y += 1;
                        } else if rope[k].y < rope[k + 1].y {
                            rope[k + 1].y -= 1;
                        } else if rope[k].x > rope[k + 1].x {
                            rope[k + 1].x += 1;
                        } else {
                            rope[k + 1].x -= 1;
                        }
                    }
                }
            }
            move_set.insert(rope[9].clone());
        }
    }

    println!("move_set = {:?}", move_set.len());
}
