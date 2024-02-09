const DAY_NUM: &str = "9";

fn next_num(nums: &[i64]) -> i64 {
    let all_zeros = nums.iter().map(|n| *n == 0).reduce(|a, e| a && e).unwrap();
    if all_zeros {
        return 0
    }
    let len = nums.len();
    let x1 = &nums[0..len-1];
    let x2 = &nums[1..len  ];
    let diff: Vec<_> = x1.iter().zip(x2).map(|(x1, x2)| x2 - x1).collect::<Vec<_>>();
    let next_diff = next_num(&*diff);
    let next_num = nums[len-1] + next_diff;
    next_num
}

fn solve(input: &str) -> [String; 2] {
    let sum = input.lines()
        .map(|l| next_num(&*l.split(' ').map(|n| n.parse::<i64>().unwrap()).collect::<Vec<_>>()))
        .sum::<i64>();
    
    [
        sum.to_string(),
        "todo".to_string()
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