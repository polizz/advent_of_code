// Sorting by strength of hands is step #1.
// This means Five of a kind, four of a kind etc.
// Each card in those grouped "types" beats the other lower groups.
// Next, each group type needs to be lexigraphically
// sorted internally by its component card labels (K, Q, J, 4 3, etc.)
// When you have final sorted order, you can run through the sorted data,
// and by *index*, multiply each *index* by the bid amount, and add that
// to the running sum.

#![allow(unused)]

mod lsd_sort;
use std::collections::HashMap;
use std::slice;

use lsd_sort::*;

fn get_total_winnings(
    input: String,
    radix_sort_map: Option<&HashMap<char, char>>,
    use_wildcards: bool,
) -> usize {
    let mut hands = parse_file(input);
    identify_hand_types(&mut hands, use_wildcards);
    let mut partitions = paritition_hands_by_type(hands);
    println!("High Card: {:#?}", &partitions[1]);
    let total_winnings = radix_sort_partitions(&mut partitions, radix_sort_map);

    total_winnings
}

fn radix_sort_partitions(
    partitions: &mut Vec<Vec<Hand>>,
    radix_sort_map: Option<&HashMap<char, char>>,
) -> usize {
    let mut idx = 1;
    let total_winnings = partitions.iter_mut().fold(0, |mut acc, mut part| {
        if part.len() > 1 {
            lsd_sort::sort_hands(&mut part, radix_sort_map);
        }
        let mut i = &mut idx;

        let part_tot_winnings = part.iter_mut().fold(0, move |mut part_tot, mut hand| {
            hand.rank = *i;
            part_tot += hand.bet * *i;
            *i += 1;

            part_tot
        });

        acc += part_tot_winnings;
        acc
    });

    total_winnings
}

fn paritition_hands_by_type(hands: Vec<Hand>) -> Vec<Vec<Hand>> {
    let mut out_vec: Vec<Vec<Hand>> = vec![vec![]; 8usize];

    hands.into_iter().fold(out_vec, |mut acc, hand| {
        match hand.hand_type {
            HandType::FiveOfAKind => {
                acc[7].push(hand);
            }
            HandType::FourOfAKind => {
                acc[6].push(hand);
            }
            HandType::FullHouse => {
                acc[5].push(hand);
            }
            HandType::ThreeOfAKind => {
                acc[4].push(hand);
            }
            HandType::TwoPair => {
                acc[3].push(hand);
            }
            HandType::OnePair => {
                acc[2].push(hand);
            }
            HandType::HighCard => {
                acc[1].push(hand);
            }
            HandType::Ungraded => {
                acc[0].push(hand);
            }
        }
        acc
    })
}

fn identify_hand_types(hands: &mut Vec<Hand>, use_wildcards: bool) {
    hands.iter_mut().for_each(|hand| {
        let mut ht: HashMap<char, usize> = HashMap::new();

        hand.cards.chars().for_each(|char| {
            ht.entry(char)
                .and_modify(|mut count| *count += 1)
                .or_insert(1);
        });

        hand.hand_type = convert_card_hash_to_handtype(ht, use_wildcards);
    })
}

fn parse_file(input: String) -> Vec<Hand> {
    let lines = input
        .lines()
        .map(|l| {
            let record: Vec<String> = l
                .split_whitespace()
                .map(|l| l.to_string())
                .collect::<Vec<String>>();

            Hand {
                cards: record[0].clone(),
                hand_type: HandType::Ungraded,
                bet: record[1].parse().expect("Should be a bet"),
                rank: 0,
            }
        })
        .collect();

    lines
}

#[derive(Debug, Clone)]
enum HandType {
    Ungraded,
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn convert_card_hash_to_handtype(
    mut card_hash: HashMap<char, usize>,
    use_wildcards: bool,
) -> HandType {
    let mut jacks_number = 0usize;
    if use_wildcards {
        if let Some((_, number)) = card_hash.get_key_value(&'J') {
            println!("Jacks: {}", number);
            jacks_number = *number;
            card_hash.remove(&'J');
        }
    }

    let card = card_hash.values().filter(|&&v| v == 1).count();
    let pair = card_hash.values().filter(|&&v| v == 2).count();
    let three_of_a_kind = card_hash.values().filter(|&&v| v == 3).count();
    let four_of_a_kind = card_hash.values().filter(|&&v| v == 4).count();
    let five_of_a_kind = card_hash.values().filter(|&&v| v == 5).count();

    if use_wildcards && jacks_number > 0 {
        if pair == 2 {
            HandType::FullHouse
        } else if three_of_a_kind == 1 {
            if jacks_number == 2 {
                HandType::FiveOfAKind
            } else {
                HandType::FourOfAKind
            }
        } else if pair == 1 {
            match jacks_number {
                1 => HandType::ThreeOfAKind,
                2 => HandType::FourOfAKind,
                3 => HandType::FiveOfAKind,
                _ => panic!("we shouldn't have enough cards to get here"),
            }
        } else if four_of_a_kind == 1 {
            HandType::FiveOfAKind
        } else if card >= 1 && card < 5 {
            match jacks_number {
                1 => HandType::OnePair,
                2 => HandType::ThreeOfAKind,
                3 => HandType::FourOfAKind,
                4 => HandType::FiveOfAKind,
                _ => panic!("we shouldn't have enough cards to get here"),
            }
        } else {
            HandType::FiveOfAKind // Five jacks
        }
    } else {
        if card == 5 {
            HandType::HighCard
        } else if five_of_a_kind == 1 {
            HandType::FiveOfAKind
        } else if pair == 2 {
            HandType::TwoPair
        } else if three_of_a_kind == 1 {
            if card == 2 {
                HandType::ThreeOfAKind
            } else {
                HandType::FullHouse
            }
        } else if pair == 1 {
            HandType::OnePair
        } else {
            HandType::FourOfAKind
        }
    }
}

#[derive(Debug, Clone)]
struct Hand {
    cards: String,
    hand_type: HandType,
    rank: usize,
    bet: usize,
}

impl Default for Hand {
    fn default() -> Self {
        Hand {
            cards: "".to_string(),
            hand_type: HandType::Ungraded,
            rank: 0,
            bet: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    fn get_card_overrides_part2() -> HashMap<char, char> {
        // ascending order is J, 2...9, T, Q, K, A
        // so we want normal alphanumerical ASCII ordering by bytecode
        // to change T, Q, and A ordering
        let mut override_sort: HashMap<char, char> = HashMap::new();
        override_sort.insert('T' as char, 'A' as char);
        override_sort.insert('A' as char, 'T' as char);
        override_sort.insert('Q' as char, 'K' as char);
        override_sort.insert('K' as char, 'L' as char);
        override_sort.insert('J' as char, '1' as char);
        // ASCII code for '1' ranks lower than any
        // alpha or other number
        override_sort
    }

    fn get_card_overrides_part1() -> HashMap<char, char> {
        // ascending order is 2...9, T, J, Q, K, A
        // so we want normal alphanumerical ASCII ordering by bytecode
        // to change T, Q, and A ordering
        let mut override_sort: HashMap<char, char> = HashMap::new();
        override_sort.insert('T' as char, 'A' as char);
        override_sort.insert('A' as char, 'T' as char);
        override_sort.insert('Q' as char, 'K' as char);
        override_sort.insert('K' as char, 'L' as char);
        override_sort
    }

    fn load_file() -> String {
        let mut file = File::open("src/fixtures/day7.txt").unwrap();
        let mut file_contents = String::new();
        let _ = file.read_to_string(&mut file_contents);
        file_contents
    }

    #[test]
    fn day7_file2() {
        let start = std::time::Instant::now();
        let input = load_file();

        let mut ht = get_card_overrides_part2();
        let total_winnings = get_total_winnings(input, Some(&ht), true);
        assert_eq!(total_winnings, 245576185usize);

        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day7_test2() {
        let start = std::time::Instant::now();
        let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

        let mut ht = get_card_overrides_part2();
        let total_winnings = get_total_winnings(input.to_string(), Some(&ht), true);

        assert_eq!(total_winnings, 5905usize);

        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day7_file() {
        let start = std::time::Instant::now();
        let input = load_file();

        let mut ht = get_card_overrides_part1();
        let total_winnings = get_total_winnings(input, Some(&ht), false);
        assert_eq!(total_winnings, 248217452usize);

        println!("Process in: {:?}", start.elapsed());
    }

    #[test]
    fn day7_test() {
        let start = std::time::Instant::now();
        let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

        let mut ht = get_card_overrides_part1();
        let total_winnings = get_total_winnings(input.to_string(), Some(&ht), false);

        assert_eq!(total_winnings, 6440usize);

        println!("Process in: {:?}", start.elapsed());
    }
}
