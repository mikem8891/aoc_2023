use std::collections::HashSet;
//use crate::util::*;

pub fn solve(input: &str) -> (u64, u64) {
    let mut sum_1  = 0;
    let mut game_cards = HashMap::new::<u64, u64>();
    for game in input.lines() {
        let game = game.split(':');
        let game_id: u64 = game.next().unwrap()
            .split(' ').last().unwrap()
            .parse().unwrap();
        game_card_num = game_cards.entry(game_id).or_insert(0);
        *game_card_num += 1;
        let mut nums = game..next().unwrap().split('|');
        let win_nums = nums.next().unwrap()
            .split(' ').filter(|s| !s.is_empty())
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<HashSet<_>>();
        let elf_nums = nums.next().unwrap()
            .split(' ').filter(|s| !s.is_empty())
            .map(|s| s.parse::<u64>().unwrap())
            .filter(|n| win_nums.contains(n))
            .collect::<Vec<_>>().len();
        if elf_nums != 0 {
            sum_1 += (1 << (elf_nums - 1)) as u64;
        }
        for new_cards in 1..=elf_nums {
            let new_card_id = game_id + new_cards;
            let new_game_card_num = game_cards.entry(new_card_id).or_insert(0);
            *new_game_card_num += game_card_num;
        }
    }
    let sum_2 = game_cards.iter().map(|(k, v)| v).sum();
    (sum_1, sum_2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input =
r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
        assert_eq!((13, 30), solve(input));
    }
}