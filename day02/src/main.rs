const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> i64 {
    let mut out: i64 = 0;
    for range in input.split(',') {
        let (start, end) = range.split_once('-').unwrap();
        let start: i64 = start.parse().unwrap();
        let end: i64 = end.parse().unwrap();
        for i in start..=end {
            let num_str = i.to_string();
            let len = num_str.len();
            if len %2 != 0 {
                continue;
            }
            let (left, right) = num_str.split_at(len / 2);
            let left: i64 = left.parse().unwrap();
            let right: i64 = right.parse().unwrap();
            if left == right {
                out += i;
            }
        }
    }
    out
}

fn part2(input: &str) -> i64 {
    let mut out: i64 = 0;
    for range in input.split(',') {
        let (start, end) = range.split_once('-').unwrap();
        let start: i64 = start.parse().unwrap();
        let end: i64 = end.parse().unwrap();
        for i in start..=end {
            let num_str = i.to_string();
            let len = num_str.len();

            for chunk_size in 1..len {

                if len % chunk_size != 0 {
                    continue;
                }

                let chunk = &num_str[0..chunk_size];
                let mut ok = true;

                for j in (0..len).step_by(chunk_size) {
                    if &num_str[j..j + chunk_size] != chunk {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    out += i;
                    break;
                }
            }
        }
    }
    out
}