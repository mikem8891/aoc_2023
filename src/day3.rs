use crate::util::*;

pub fn part_nums(eng_schem: &str) -> u64 {
    let mut sum = 0;
    let eng_schem: Vec<&str> = eng_schem.lines().collect();
    let in_range = |r, c| { 
        c >= 0 && c < eng_schem[r].len() &&
        r >= 0 && r < eng_schem.len()
    }
    for num_row in 0..eng_schem.len() {
        let start = 0;
        while let Some(num_col) = eng_schem[num_row][start..].find(is_digit) {
            
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