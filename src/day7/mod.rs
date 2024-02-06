const DAY_NUM: &str = "7";

use std::collections::BTreeMap;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5]
}

impl HandType {
    fn new(cards: &[Card; 5]) -> HandType {
        let mut count = BTreeMap::<_, u8>::new();
        for card in cards.iter() {
            count.entry(card)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
        let mut count: Vec<_> = count.into_iter()
            .map(|(_card, count)| count)
            .collect();
        count.sort();
        use HandType::*;
        match count[..] {
            [5] => FiveKind,
            [.., 4] => FourKind,
            [2, 3] => FullHouse,
            [.., 3] => ThreeKind,
            [.., 2, 2] => TwoPair,
            [.., 2] => OnePair,
            [..] => HighCard
        }
    }
}

impl Card {
    fn new(c: char) -> Card {
        use Card::*;
        match c {
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            'T' => Ten,
            'J' => Jack,
            'Q' => Queen,
            'K' => King,
            'A' => Ace,
            c => panic!("'{c}' is an invalid char for a Card")
        }
    }
}

impl Hand {
    fn new(text: &str) -> Hand {
        let mut cards = [Card::Ace; 5];
        for (pos, card) in text.chars().enumerate() {
            cards[pos] = Card::new(card);
        }
        let hand_type = HandType::new(&cards);
        Hand {hand_type, cards}
    }
}

fn solve(input: &str) -> [String; 2] {
    let mut hands: Vec<(Hand, u32)> = vec![];
    for line in input.lines() {
        let (hand, bid) = line.split_once(' ').unwrap();
        let hand = Hand::new(hand);
        let bid = bid.parse().unwrap();
        hands.push((hand, bid));
    }
    hands.sort_by_key(|(h, _b)| h.clone());
    let winnings: u32 = hands.iter().zip(1..)
        .map(|((_h, b), r)| r * b)
        .sum();
    [
        winnings.to_string(), 
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