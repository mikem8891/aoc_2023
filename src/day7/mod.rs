const DAY_NUM: &str = "7";

use std::collections::BTreeMap;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
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
    Joker = 1,
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

#[derive(PartialEq, Eq, PartialOrd, Ord)]
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
        let joker_count = count.remove(&Card::Joker).unwrap_or(0);
        let mut count: Vec<_> = count.into_values()
            .collect();
        count.sort();
        match count.last_mut() {
            Some(c) => *c += joker_count,
            None => count.push(joker_count)
        };
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
    
    fn new_p2(c: char) -> Card {
        match c {
            'J' => Card::Joker,
            c => Card::new(c)
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
    
    fn new_p2(text: &str) -> Hand {
        let mut cards = [Card::Ace; 5];
        for (pos, card) in text.chars().enumerate() {
            cards[pos] = Card::new_p2(card);
        }
        let hand_type = HandType::new(&cards);
        Hand {hand_type, cards}
    }
}

fn solve(input: &str) -> [String; 2] {
    let mut hands: Vec<(Hand, u32)> = vec![];
    let mut hands_p2: Vec<(Hand, u32)> = vec![];
    for line in input.lines() {
        let (hand, bid) = line.split_once(' ').unwrap();
        let hand_p2 = Hand::new_p2(hand);
        let hand = Hand::new(hand);
        let bid = bid.parse().unwrap();
        hands.push((hand, bid));
        hands_p2.push((hand_p2, bid));
    }
    let cmp_hands = |(h1, _): &(Hand, u32), (h2, _): &(Hand, u32)| h1.cmp(h2);
    hands.sort_by(cmp_hands);
    hands_p2.sort_by(cmp_hands);
    let winnings: u32 = hands.iter().zip(1..)
        .map(|((_h, b), r)| r * b)
        .sum();
    let winnings_p2: u32 = hands_p2.iter().zip(1..)
        .map(|((_h, b), r)| r * b)
        .sum();
    [
        winnings.to_string(), 
        winnings_p2.to_string()
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