const INPUT: &str = include_str!("../input.txt");

fn main() {
    let grid = parse_input(INPUT);
    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(grid.clone()));
}

const DIRS: [(isize, isize); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1), (0, 1),
    (1, -1),  (1, 0),  (1, 1),
];

fn part1(grid: &Vec<Vec<u8>>) -> i64 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut out = 0;

    for r in 0..rows {
        for c in 0..cols {
            let current = grid[r][c];
            if current as char != '@' {
                continue;
            }

            let mut adjacents = 0;

            for &(dr, dc) in DIRS.iter() {
                let nrow = r as isize + dr;
                let ncol = c as isize + dc;

                if nrow >= 0 && nrow < rows as isize && ncol >= 0 && ncol < cols as isize {
                    let adjacent_char = grid[nrow as usize][ncol as usize];

                    if adjacent_char == b'@' {
                        adjacents += 1;
                    }
                }
            }
            if adjacents < 4 {
                out += 1;
            }
        }
    }
    out
}

fn part2(mut grid: Vec<Vec<u8>>) -> i64 {
    let mut removed = 0;

    loop {
        let rows = grid.len();
        let cols = grid[0].len();

        let mut changed = false;
        let mut next_grid = grid.clone();

        for r in 0..rows {
            for c in 0..cols {
                let current = grid[r][c];
                if current as char != '@' {
                    continue;
                }

                let mut adjacent = 0;
                for &(dr, dc) in DIRS.iter() {
                    let nrow = r as isize + dr;
                    let ncol = c as isize + dc;
                    if nrow >= 0 && nrow < rows as isize && ncol >= 0 && ncol < cols as isize {
                        let adjacent_char = grid[nrow as usize][ncol as usize];
                        if adjacent_char == b'@' {
                            adjacent += 1;
                        }
                    }
                }

                if adjacent < 4 {
                    removed += 1;
                    next_grid[r][c] = b'.';
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
        grid = next_grid;
    }
    removed
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines()
        .map(|line| line.as_bytes().to_vec())
        .collect()
}