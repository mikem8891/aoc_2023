use crate::util::*;

pub fn part_nums(eng_schem: &str) -> u64 {
    let mut sum = 0;
    let eng_schem: Vec<&str> = eng_schem.lines().collect();
    let in_range = |r: usize, c: usize| { 
        c >= 0 && c < eng_schem[r].len() &&
        r >= 0 && r < eng_schem.len()
    };
    for num_row in 0..eng_schem.len() {
        let mut end = 0;
        while let Some(start) = eng_schem[num_row][end..].find(is_digit) {
            end = start + eng_schem[num_row][start..].find(is_not_digit).unwrap();
            let mut part = false;
            for c in ((start - 1)..(end + 1)).filter(|c| in_range(num_row - 1, *c)) {
                part = !matches!(eng_schem[num_row].as_bytes()[c], b'.' | b'0'..=b'9');
            }
        }
    }
    
    sum
}


#[cfg(test)]
mod test {
    
    #[test]
    fn example() {
        
    }
}