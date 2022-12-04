use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file = include_str!("./input.txt");
    let file: String = file.into();
    let mut sum = 0;

    let elf_pairs: Vec<String> = file.trim_end().split('\n').map(|x| x.to_string()).collect();

    for pair in elf_pairs.as_slice() {
        let mut ranges: Vec<Vec<i32>> = Vec::new();
        let pair_range: Vec<String> = pair.split(',').map(|x| x.to_string()).collect();

        for range in pair_range {
            let range: Vec<String> = range.split('-').map(|x| x.to_string()).collect();
            let range: Vec<i32> = range.iter().map(|x| x.parse::<i32>().unwrap()).collect();
            ranges.push(range);
        }

        let range1 = ranges[0][0]..=ranges[0][1];
        let range2 = ranges[1][0]..=ranges[1][1];

        if range1.contains(&ranges[1][0]) || range1.contains(&ranges[1][1]) {
            sum += 1;
        } else if range2.contains(&ranges[0][0]) || range2.contains(&ranges[0][1]) {
            sum += 1;
        }
    }

    println!("{:?}", sum);
    Ok(())
}
