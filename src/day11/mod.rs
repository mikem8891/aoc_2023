const DAY_NUM: &str = "11";

fn solve(input: &str) -> [String; 2] {
    let star_map: Vec<_> = input.lines().map(|l| l.as_bytes()).collect();
    let row_num = star_map.len();
    let col_num = star_map[0].len();
    let mut ex_rows = Vec::with_capacity(row_num);
    let mut ex_cols = Vec::with_capacity(col_num);
    ex_rows.push(0);
    for r in 1..row_num {
        let empty = star_map[r].iter()
            .map(|s| s == b'.')
            .fold(true, |acc, s| acc && s);
        let new_last = ex_row.last().unwrap() +
            if empty {2} else {1};
        ex_rows.push(new_last);
    }
    ex_cols.push(0);
    for c in 1..col_num {
        let empty = (0..row_num).iter()
            .map(|r| star_map[r][c] == b'.')
            .fold(true, |acc, s| acc && s);
        let new_last = ex_cols[c-1] +
            if empty {2} else {1};
        ex_cols.push(new_last);
    }
    let mut galaxies = vec![];
    for r in 0..row_num {
        for c in 0..col_num {
            if star_map[r][c] == b'#' {
                let galaxy = (ex_rows[r], ex_cols[c]);
                galaxies.push(galaxy);
            }
        }
    }
    let galaxy_num = galaxies.len();
    let mut sum = 0;
    for from in 1..galaxy_num {
        for to in 0..from {
            let (r_t, c_t) = galaxies[to];
            let (r_f, c_f) = galaxies[from];
            let dist = r_t.abs_diff(r_f) + c_t.abs_diff(c_f);
            sum =+ dist;
        }
    }
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