const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part 1: {}", part1(parse_input(INPUT)));
    println!("Part 2: {}", part2(parse_input(INPUT)));
}

fn part1((ranges, ids): (Vec<(usize, usize)>, Vec<usize>)) -> usize {
    let mut out = 0;

    for id in ids {
        if ranges.iter()
        .any(|(s, e)| id >= *s && id <= *e) {
            out += 1;
        }
    }
    out
}

fn part2((mut ranges, _ids): (Vec<(usize, usize)>, Vec<usize>)) -> usize {
    let mut out = 0;

    ranges.sort_unstable_by_key(|(s,_)| *s);

    let mut cur_start = ranges[0].0;
    let mut cur_end = ranges[0].1;

    for (s,e) in ranges.into_iter().skip(1) {
        if s <= cur_end + 1 {
            if e > cur_end {
                cur_end = e;
            }
        } else {
            out += cur_end - cur_start + 1;
            cur_start = s;
            cur_end = e;
        }
    }
    out += cur_end - cur_start + 1;
    out
}

fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    let parts: Vec<&str> = input.split("\r\n\r\n").collect();

    let mut ranges= Vec::new();
    let mut ids= Vec::new();

    for line in parts[0].lines() {
        let (a, b) = line.split_once('-').unwrap();
        ranges.push((a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()));
    }
    for n in parts[1].lines() {
        let id: usize = n.parse().unwrap();
        ids.push(id);
    }
    (ranges, ids)
}
