const DAY_NUM: &str = "6";

fn solve(input: &str) -> [String; 2] {
    let mut lines = input.lines();
    let time_str = lines.next().unwrap()
        .split_once(':').unwrap().1;
    let times_iter = time_str.split(' ')
        .filter_map(|s| s.parse::<u64>().ok());
    let dist_str = lines.next().unwrap()
        .split_once(':').unwrap().1;
    let dists_iter = dist_str.split(' ')
        .filter_map(|s| s.parse::<u64>().ok());
    let races: Vec<(_, _)> = times_iter.zip(dists_iter).collect();
    let time_p2 = time_str.chars().filter(char::is_ascii_digit).collect::<String>()
        .parse::<u64>().unwrap();
    let dist_p2 = dist_str.chars().filter(char::is_ascii_digit).collect::<String>()
        .parse::<u64>().unwrap();
    // d = ht * (t - ht)
    // ht = (t / 2) ± √((t / 2)² - d)
    let hold_time_rng = |&(t, d)| {
        let hlf_t = (t as f64) / 2.0;
        let sq = hlf_t * hlf_t - (d as f64);
        if sq <= 0.0 {
            panic!("no solution for a time of {t} and distance of {d}");
        }
        let sqrt = f64::sqrt(sq);
        let start = hlf_t - sqrt;
        let end   = hlf_t + sqrt;
        let start = (start + 1.0).floor() as u64;
        let end   = (end   - 1.0).ceil()  as u64;
        (start, end)
    };
    let ways_to_win: u64 = races.iter().map(hold_time_rng)
        .map(|(s, e)| e - s + 1)
        .product();
    let (start_p2, end_p2) = hold_time_rng(&(time_p2, dist_p2));
    let ways_to_win_p2 = end_p2 - start_p2 + 1;
    [
        ways_to_win.to_string(), 
        ways_to_win_p2.to_string()
    ]
}

pub fn main() {
    println!("Day {DAY_NUM}");
    let path = format!("./src/day{DAY_NUM}/input.txt");
    let input = std::fs::read_to_string(path).unwrap();
    let [part_1, part_2] = solve(&input);
    println!("part 1 is {part_1}");
    println!("part 2 is {part_2}");
}

#[cfg(test)]
mod test;