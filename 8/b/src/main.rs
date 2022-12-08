fn main() {
    let file = include_str!("./input.txt");

    let file_vec: Vec<String> = file.trim_end().split("\n").map(|x| x.to_string()).collect();

    let mut forest: Vec<Vec<usize>> = Vec::new();
    let mut scenic_scores: Vec<usize> = Vec::new();

    for row in &file_vec {
        let tree_line: Vec<usize> = row
            .split("")
            .filter_map(|token| token.parse().ok())
            .collect();
        forest.push(tree_line);
    }

    let grid_width = forest[0].len();

    for (tree_line, row) in forest.iter().enumerate() {
        if tree_line == 0 || tree_line == forest.len() - 1 {
            continue;
        }
        for (index, tree) in row.iter().enumerate() {
            if index == 0 || index == grid_width - 1 {
                continue;
            }

            let mut visible: Vec<usize> = Vec::new();

            let l = row[0..index].iter();
            let mut count = 0;
            for l_tree in l.rev() {
                if l_tree >= tree {
                    count += 1;
                    break;
                } else {
                    count += 1;
                }
            }
            visible.push(count);

            let r = row[index + 1..].iter();
            let mut count = 0;
            for r_tree in r {
                if r_tree >= tree {
                    count += 1;
                    break;
                } else {
                    count += 1;
                }
            }
            visible.push(count);

            let u = forest.iter().take(tree_line);
            let mut count = 0;
            for u_tree in u.rev() {
                if u_tree[index] >= *tree {
                    count += 1;
                    break;
                } else {
                    count += 1;
                }
            }
            visible.push(count);

            let d = forest.iter().skip(tree_line + 1);
            let mut count = 0;
            for d_tree in d {
                if d_tree[index] >= *tree {
                    count += 1;
                    break;
                } else {
                    count += 1;
                }
            }
            visible.push(count);

            scenic_scores.push(visible[0] * visible[1] * visible[2] * visible[3]);
        }
    }

    scenic_scores.sort();
    println!("scenic_scores = {:?}", scenic_scores.last().unwrap());
}
