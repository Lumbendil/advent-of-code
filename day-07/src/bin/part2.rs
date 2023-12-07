use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use crate::HandRank::*;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    process(handle, io::stdout());
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
enum HandRank {
    FiveKind = 7,
    FourKind = 6,
    FullHouse = 5,
    ThreeKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
enum Card {
    VA = 13,
    VK = 12,
    VQ = 11,
    VT = 9,
    V9 = 8,
    V8 = 7,
    V7 = 6,
    V6 = 5,
    V5 = 4,
    V4 = 3,
    V3 = 2,
    V2 = 1,
    VJ = 0,
}

impl Card {
    fn from_char(c: char) -> Card {
        match c {
            'A' => Card::VA,
            'K' => Card::VK,
            'Q' => Card::VQ,
            'J' => Card::VJ,
            'T' => Card::VT,
            '9' => Card::V9,
            '8' => Card::V8,
            '7' => Card::V7,
            '6' => Card::V6,
            '5' => Card::V5,
            '4' => Card::V4,
            '3' => Card::V3,
            '2' => Card::V2,
            _ => panic!("unexpected card")
        }
    }
}

#[derive(Debug, Ord, Eq, PartialEq)]
struct Hand {
    cards: [Card;5],
    hand_rank: HandRank,
    bid: u32,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if let Some(result) = self.hand_rank.partial_cmp(&other.hand_rank) {
            if !result.is_eq() {
                return Some(result)
            }
        }

        for (left, right) in self.cards.iter().zip(&other.cards) {
            if let Some(result) = left.partial_cmp(&right) {
                if !result.is_eq() {
                    return Some(result)
                }
            }
        }

        Some(Equal)
    }
}

fn calculate_card_counts_rank(count: HashMap<Card, u8>) -> HandRank {
    let mut best_rank = HighCard;

    for (card, &i) in &count {
        if card == &Card::VJ {
            continue;
        }

        if i == 3 && best_rank == OnePair || i == 2 && best_rank == ThreeKind {
            return FullHouse
        }

        if i == 2 && best_rank == OnePair {
            best_rank = TwoPair;
            break;
        }

        let rank = match i {
            5 => return FiveKind,
            4 => FourKind,
            3 => ThreeKind,
            2 => OnePair,
            1 => HighCard,
            _ => panic!("unexpected card count")
        };

        if rank > best_rank {
            best_rank = rank;
        }
    }

    if let Some(joker_count) = count.get(&Card::VJ) {
        best_rank = match joker_count {
            4|5 => FiveKind,
            3 => match best_rank {
                OnePair => FiveKind,
                _ => FourKind,
            },
            2 => match best_rank {
                ThreeKind => FiveKind,
                OnePair => FourKind,
                _ => ThreeKind,
            }
            1 => match best_rank {
                FourKind => FiveKind,
                TwoPair => FullHouse,
                ThreeKind => FourKind,
                OnePair => ThreeKind,
                _ => OnePair,
            }
            _ => panic!("unexpected card count")
        };
    }

    best_rank
}

fn parse_line(input: &str) -> Hand {
    let mut chars = input.chars();
    let mut cards= [Card::V2; 5];
    let mut card_counts = HashMap::new();

    for i in 0..5 {
        let card = Card::from_char(chars.next().expect("card"));
        cards[i] = card;
        card_counts.entry(card)
            .and_modify(|e| { *e += 1 })
            .or_insert(1);
    }

    chars.next();
    let bid = chars.as_str().parse::<u32>().expect("Expected a number");
    let hand_rank = calculate_card_counts_rank(card_counts);

    Hand {
        cards,
        hand_rank,
        bid,
    }
}

fn process(input: impl BufRead, mut output: impl Write) {
    let mut data: Vec<Hand> = input.lines().map(|line| parse_line(line.unwrap().as_str())).collect();

    data.sort();

    let value: u32 = data.iter().enumerate()
        .map(|(key, hand)| (key as u32 + 1) * hand.bid)
        .sum();

    output.write(value.to_string().as_bytes()).unwrap();
}