const DAY_NUM: &str = "6";

fn solve(input: &str) -> [String; 2] {
    let lines: Vec<&str> = input.lines();
    let times_iter: Vec<u32> = lines.next().unwrap()
        .split_once(':').unwrap().1
        .split(' ').filter_map(|s| s.parse::<u32>().ok());
    let dists_iter: Vec<u32> = lines.next().unwrap()
        .split_once(':').unwrap().1
        .split(' ').filter_map(|s| s.parse::<u32>().ok());
    let races = times_iter.zip(dist_iter).collect();
    // d = ht * (t - ht)
    // ht = (t / 2) ± √((t / 2)² - d)
    let hold_time_rng = |(t, d)| {
        let hlf_t = (t as f64) / 2;
        let sq = hlf_t * hlf_t - (d as f64);
        if sq <= 0 {
            panic!("no solution for a time of {t} and distance of {d}");
        }
        let sqrt = f64::sqrt(rad);
        let start = hlf_t - sqrt(rad);
        let end   = hlf_t + sqrt(rad);
        let start = (start + 1.0).floor() as u32;
        let end   = (end   - 1.0).ceil()  as u32;
        (start, end)
    }
    let ways_to_win = races.iter().map(hold_time_rng)
        .map(|(s, e)| e - s + 1)
        .product();
    [
        ways_to_win.to_string(), 
        "todo".to_owned()
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