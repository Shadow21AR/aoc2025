const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> i64 {
    let mut c_u_r_p_o_s: i32 = 50;
    let mut c_o_u_n_t_z_e_r_o: i64 = 0;

    for line in input.lines() {
        let dir = line.chars().next().unwrap();
        let num: u32 = line[1..].parse().unwrap();

        match dir {
            'L' => c_u_r_p_o_s -= num as i32,
            'R' => c_u_r_p_o_s += num as i32,
            _ => unreachable!(),
        }
        c_u_r_p_o_s = (c_u_r_p_o_s % 100 + 100) % 100;

        if c_u_r_p_o_s == 0 {
            c_o_u_n_t_z_e_r_o += 1;
        }
    }
    c_o_u_n_t_z_e_r_o
}

fn part2(input: &str) -> i64 {
    let mut c_u_r_p_o_s: i32 = 50;
    let mut c_o_u_n_t_z_e_r_o: i64 = 0;

    for line in input.lines() {
        let dir = line.chars().next().unwrap();
        let steps: i32 = line[1..].parse().unwrap();

        let movement = match dir {
            'L' => -steps,
            'R' =>  steps,
            _ => unreachable!(),
        };

        let start = c_u_r_p_o_s;
        let steps = movement.abs();

        let t0 = match (start == 0, movement > 0) {
            (true,  _) => 100,
            (false, true) => 100 - start,
            (false, false) => start,
        };

        if t0 <= steps {
            c_o_u_n_t_z_e_r_o += 1 + ((steps - t0) / 100) as i64;
        }

        c_u_r_p_o_s = ((c_u_r_p_o_s + movement) % 100 + 100) % 100;
    }

    c_o_u_n_t_z_e_r_o
}
