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

    let mut head = Pos { x: 0, y: 0 };
    let mut tail = Pos { x: 0, y: 0 };

    for step in &move_vec {
        for _i in 0..step.steps {
            if step.direction == "L" {
                head.x -= 1;
            }
            if step.direction == "R" {
                head.x += 1;
            }
            if step.direction == "U" {
                head.y += 1;
            }
            if step.direction == "D" {
                head.y -= 1;
            }

            let d_x = head.x.abs_diff(tail.x);
            let d_y = head.y.abs_diff(tail.y);

            if d_x > 1 || d_y > 1 {
                if tail.x != head.x && tail.y != head.y {
                    if head.x > tail.x && head.y > tail.y {
                        tail.x += 1;
                        tail.y += 1;
                    } else if head.x < tail.x && head.y < tail.y {
                        tail.x -= 1;
                        tail.y -= 1;
                    } else if head.x > tail.x && head.y < tail.y {
                        tail.x += 1;
                        tail.y -= 1;
                    } else {
                        tail.x -= 1;
                        tail.y += 1;
                    }
                } else {
                    if head.y > tail.y {
                        tail.y += 1;
                    } else if head.y < tail.y {
                        tail.y -= 1;
                    } else if head.x > tail.x {
                        tail.x += 1;
                    } else {
                        tail.x -= 1;
                    }
                }
            }
            move_set.insert(tail.clone());
        }
    }

    println!("move_set = {:?}", move_set.len());
}
