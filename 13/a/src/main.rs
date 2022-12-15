use std::error::Error;

// struct Message {
//     left: Vec<Packets>,
//     right: Vec<Packets>,
// }
//
// #[derive(Debug)]
// enum Packets {
//     Int(i8),
//     List(Vec<usize>),
// }

fn main() -> Result<(), Box<dyn Error>> {
    // let file = include_str!("./input.test");
    let file = include_str!("./input.txt");

    let file_vec: Vec<_> = file.trim_end().split("\n\n").collect();

    let mut right_count = 0;

    let mut comp_index = 1;
    let mut correct_indicies: Vec<i32> = Vec::new();

    for file in &file_vec {
        let spam: Vec<_> = file.split("\n").map(|el| el.to_string()).collect();
        let left = &spam[0];
        let left = left[1..left.len() - 1].to_string();
        let right = &spam[1];
        let right = right[1..right.len() - 1].to_string();

        let left: Vec<i32> = left
            .split("")
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();
        let right: Vec<i32> = right
            .split("")
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();
        println!("l and r = {:?} : {:?}", left, right);

        if left.len() == 0 && left.len() < right.len() {
            right_count += 1;
            correct_indicies.push(comp_index);
            comp_index += 1;
            continue;
        }

        for (index, l_val) in left.iter().enumerate() {
            println!("l_val = {:?}", l_val);
            if right.get(index).is_some() {
                if l_val < right.get(index).unwrap() {
                    println!("right.get(index) = {:?}", right.get(index));
                    right_count += 1;
                    correct_indicies.push(comp_index);
                    break;
                } else if left.len() < right.len() && right[index] >= *l_val {
                    right_count += 1;
                    println!("left = {:?}", left.len());
                    correct_indicies.push(comp_index);
                    break;
                }
            }
        }
        println!("right_count = {:?}", right_count);
        comp_index += 1;
    }

    println!("correct_indicies = {:?}", correct_indicies);
    let sum: i32 = correct_indicies.iter().sum();
    println!("sum of correct_indicies = {:?}", sum);

    Ok(())
}
