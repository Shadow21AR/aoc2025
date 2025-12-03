const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> i64{
    let mut joltage: i64 = 0;

    for bank in input.lines() {
        let batteries: Vec<i64> = bank
        .bytes()
        .map(|b| (b - b'0') as i64)
        .collect();

        let mut max_x = -1;
        let mut max_x_index = 0;

        for (i, &v) in batteries[..batteries.len() - 1].iter().enumerate() {
            if v > max_x {
                max_x = v;
                max_x_index = i;
            }
        }

        let max_y = batteries[max_x_index+1..]
        .iter()
        .max()
        .unwrap();

        let max_jolt = format!("{}{}", max_x, max_y)
        .parse::<i64>()
        .unwrap();

        joltage += max_jolt;
    }
    joltage
}

fn part2(input: &str) -> i64{
    let mut joltage: i64 = 0;

    for bank in input.lines() {
        let batteries: Vec<i64> = bank
        .bytes()
        .map(|b| (b - b'0') as i64)
        .collect();

        let mut result_num: Vec<i64> = Vec::new();
        let mut slice = &batteries[..];

        for i in 0..12 {
            if slice.is_empty(){
                result_num.push(0);
                continue;
            }

            let mut max_x = -1;
            let mut max_x_index = 0;

            let need_left = 12-i;
            let allowed_end = slice.len() - need_left;

            for (j, &v) in slice[..=allowed_end].iter().enumerate() {
                if v > max_x {
                    max_x = v;
                    max_x_index = j;
                }
            }

            result_num.push(max_x);
            slice = &slice[max_x_index + 1 ..];
        }
        let mut num = 0;
        for d in result_num {
            num = num * 10 + d;
        }
        joltage += num;
    }
    joltage
}