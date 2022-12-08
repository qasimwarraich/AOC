fn main() {
    let file = include_str!("./input.txt");

    let file_vec: Vec<String> = file.trim_end().split("\n").map(|x| x.to_string()).collect();

    let mut forest: Vec<Vec<usize>> = Vec::new();
    let mut tree_count = 0 as usize;

    for row in &file_vec {
        let tree_line: Vec<usize> = row
            .split("")
            .filter_map(|token| token.parse().ok())
            .collect();
        forest.push(tree_line);
    }

    for (tree_line, row) in forest.iter().enumerate() {
        if tree_line == 0 || tree_line == forest.len() - 1 {
            tree_count += row.len();
            continue;
        }
        for (index, tree) in row.iter().enumerate() {
            if index == 0 || index == forest.len() - 1 {
                tree_count += 1;
                continue;
            }
            let mut visible: Vec<bool> = Vec::new();

            let status = row[0..index].iter().all(|x| x < tree);
            visible.push(status);

            let status = row[index + 1..].iter().all(|x| x < tree);
            visible.push(status);

            let status = forest.iter().take(tree_line).all(|x| x[index] < *tree);
            visible.push(status);

            let status = forest.iter().skip(tree_line + 1).all(|x| x[index] < *tree);
            visible.push(status);

            if visible.iter().any(|x| *x == true) {
                tree_count += 1;
            }
        }
    }
    println!("tree_count = {:?}", tree_count);
}
