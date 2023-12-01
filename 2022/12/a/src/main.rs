use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    error::Error,
};

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
struct Node {
    cost: i32,
    pos: Pos,
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
struct Pos {
    x: usize,
    y: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = include_str!("./input.txt");

    let (map, rows, cols, start, end) = parse_file(file);

    let mut pq = BinaryHeap::new();
    let mut visited = HashSet::new();

    pq.push(Node {
        cost: 0,
        pos: start,
    });

    while let Some(Node { pos, cost }) = pq.pop() {
        if pos == end {
            println!("cost = {:?}", cost);
            return Ok(());
        }

        let current_height = map[pos.y][pos.x];
        let neighbours = pos.get_neighbours(rows, cols);
        let candidates: Vec<_> = neighbours
            .iter()
            .filter(|candidate| {
                let height = map[candidate.y][candidate.x];
                height <= current_height || height == current_height + 1
            })
            .collect();

        for candidate in candidates {
            if visited.insert(*candidate) {
                pq.push(Node {
                    cost: cost + 1,
                    pos: *candidate,
                })
            }
        }
    }

    Ok(())
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Pos {
    fn get_neighbours(&self, rows: usize, cols: usize) -> Vec<Pos> {
        let mut neighbours: Vec<Pos> = Vec::new();

        if self.x >= 1 {
            neighbours.push(Pos {
                x: self.x - 1,
                y: self.y,
            });
        }

        if self.x < cols - 1 {
            neighbours.push(Pos {
                x: self.x + 1,
                y: self.y,
            });
        }

        if self.y >= 1 {
            neighbours.push(Pos {
                x: self.x,
                y: self.y - 1,
            });
        }

        if self.y < rows - 1 {
            neighbours.push(Pos {
                x: self.x,
                y: self.y + 1,
            });
        }
        return neighbours;
    }
}

fn find_start_and_end(grid: &Vec<Vec<char>>) -> (Pos, Pos) {
    let mut start = Pos { x: 0, y: 0 };
    let mut end = Pos { x: 0, y: 0 };
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'S' {
                start.x = x;
                start.y = y;
            }
            if grid[y][x] == 'E' {
                end.x = x;
                end.y = y;
            }
        }
    }
    return (start, end);
}

fn parse_file(file: &str) -> (Vec<Vec<usize>>, usize, usize, Pos, Pos) {
    let file_vec: Vec<String> = file
        .trim_end()
        .split("\n")
        .map(|el| el.to_string())
        .collect();

    let cols = file_vec[0].len();
    let rows = file_vec.len();

    let mut grid: Vec<Vec<char>> = Vec::new();
    for row in file_vec {
        let grid_row: Vec<char> = row.chars().collect();
        grid.push(grid_row);
    }

    let mut map: Vec<Vec<usize>> = Vec::new();
    for row in &grid {
        let mut map_row: Vec<usize> = Vec::new();
        for char in row {
            let val = find_char_val(char.to_owned());
            map_row.push(val);
        }
        map.push(map_row)
    }

    let (start, end) = find_start_and_end(&grid);

    return (map, rows, cols, start, end);
}

fn find_char_val(a: char) -> usize {
    if a == 'S' {
        return 0;
    }
    if a == 'E' {
        return 25;
    }

    let mut pos = 0;
    let mut i = 0;
    for char in 'a'..='z' {
        if char == a {
            pos = i;
        }
        i += 1;
    }
    return pos;
}
