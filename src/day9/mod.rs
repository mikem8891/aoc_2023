const DAY_NUM: &str = "9";

fn extrap_num(nums: &[i64]) -> (i64, i64) {
    let all_zeros = nums.iter()
        .map(|n| *n == 0).reduce(|a, e| a && e).unwrap();
    if all_zeros {
        return (0, 0)
    }
    let len = nums.len();
    let x1 = &nums[0..len-1];
    let x2 = &nums[1..len  ];
    let diff: Vec<_> = x1.iter().zip(x2).map(|(x1, x2)| x2 - x1).collect::<Vec<_>>();
    let (prev_diff, next_diff) = extrap_num(&*diff);
    let next_num = nums[len-1] + next_diff;
    let prev_num = nums[0]      - prev_diff;
    (prev_num, next_num)
}

fn solve(input: &str) -> [String; 2] {
    let num_lists = input.lines()
        .map(|l| l.split(' ').map(|n| n.parse::<i64>().unwrap()).collect::<Vec<_>>());
    let sums = num_lists
        .map(|l| extrap_num(&*l))
        .reduce(|(sp, sn), (p, n)| ((sp + p), (sn + n))).unwrap();
    [
        sums.1.to_string(),
        sums.0.to_string()
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