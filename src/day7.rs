use std::{process, fs, fmt::Debug, collections::{BTreeSet, HashMap}, cmp::Ordering};

use crate::UnwrapOrExit;

const PART_2: bool = true;

pub fn main(args: Vec<String>) {
	
	let syntax = format!("Syntax: {} {} <file path>", args[0], args[1]);

	if args.len() != 3 {
		eprintln!("{syntax}");
		process::exit(1);
	}	let content = fs::read_to_string(&args[2]).unwrap_or_exit("Could not read file content as Utf-8 string", 1);

	let hands: BTreeSet<CardHand> = content
		.lines()
		.map(|l| {
			let parts = l
				.split_once(' ')
				.unwrap_or_exit(&format!("Unable to parse line {l}"), 1);

			let mut cards = parts.0
				.bytes()
				.map(|c| c.try_into().unwrap_or_exit(&format!("Could not parse {} as cards", parts.0), 1));
			
			if cards.len() != 5 {
				eprintln!("Found cards with invalid length {} (expected: 5): {}", cards.len(), parts.0);
				process::exit(1);
			};
			CardHand {
				cards: [
					cards.next().expect("Cards should be length 5"),
					cards.next().expect("Cards should be length 5"),
					cards.next().expect("Cards should be length 5"),
					cards.next().expect("Cards should be length 5"),
					cards.next().expect("Cards should be length 5")
				],
				bid: parts.1.trim().parse::<u32>().unwrap_or_exit(&format!("Could not read bit as integer: {}", parts.1), 1)
			}
		})
		.collect();

	let mut sum = 0;
	for (i, hand) in hands.iter().enumerate() {
		sum += (i+1) as u32 * hand.bid;
	}

	println!("{hands:#?}");
	println!("{sum}");
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct CardHand {
	cards: [Card; 5],
	bid: u32
}

impl CardHand {
	fn kind(&self) -> HandType {
		let mut card_counts = HashMap::with_capacity(5);
		for card in self.cards {
			let amount = card_counts.entry(card).or_insert(0);
			*amount += 1;
		}
		if PART_2 && card_counts.len() > 1 {
			if let Some(c) = card_counts.remove(&Card::Jack) {
				let (_, value) = card_counts.iter_mut().max_by(|(_, first), (_, second)| first.cmp(second)).expect("Should have at least one element");
				*value += c;
			}
		}
		let sorted_counts: BTreeSet<u32> = card_counts.values().copied().collect();
		match sorted_counts.last().expect("Card counts should have at least one element") {
			5 => HandType::FiveOfAKind,
			4 => HandType::FourOfAKind,
			3 => if card_counts.len() == 2 {
				HandType::FullHouse
			} else {
				HandType::ThreeOfAKind
			}
			2 => if card_counts.len() == 3 {
				HandType::TwoPair
			} else {
				HandType::OnePair
			}
			_ => HandType::HighCard
		}
	}
}

impl Ord for CardHand {
    fn cmp(&self, other: &Self) -> Ordering {
		if self.kind() != other.kind() {
			return self.kind().cmp(&other.kind());
		} else {
			for i in 0..5 {
				if self.cards[i] != other.cards[i] {
					return self.cards[i].cmp(&other.cards[i]);
				}
			}
			return Ordering::Equal;
		}
    }
}

impl PartialOrd for CardHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Debug for CardHand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CardHand").field("cards", &self.cards).field("bid", &self.bid).field("kind", &self.kind()).finish()
    }
}

#[repr(u8)]
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum Card {
	Ace = b'A',
	King = b'K',
	Queen = b'Q',
	Jack = b'J',
	Ten = b'T',
	Nine = b'9',
	Eight = b'8',
	Seven = b'7',
	Six = b'6',
	Five = b'5',
	Four = b'4',
	Three = b'3',
	Two = b'2',
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Card {
	pub fn order_index(&self) -> u8 {
		match self {
			Card::Ace => 12,
			Card::King => 11,
			Card::Queen => 10,
			Card::Jack => if PART_2 { 0 } else { 9 },
			Card::Ten => if PART_2 { 9 } else { 8 },
			v=> *v as u8 - if PART_2 { b'1' } else { b'2' }
		}
	}
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.order_index().cmp(&other.order_index())
    }
}

impl TryFrom<u8> for Card {
    type Error = String;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
		match v {
			b'A' => Ok(Card::Ace),
			b'K' => Ok(Card::King),
			b'Q' => Ok(Card::Queen),
			b'J' => Ok(Card::Jack),
			b'T' => Ok(Card::Ten),
			b'9' => Ok(Card::Nine),
			b'8' => Ok(Card::Eight),
			b'7' => Ok(Card::Seven),
			b'6' => Ok(Card::Six),
			b'5' => Ok(Card::Five),
			b'4' => Ok(Card::Four),
			b'3' => Ok(Card::Three),
			b'2' => Ok(Card::Two),
			b => Err(format!("Invalid value for card {}", b as char))
		}
    }
}

#[repr(u8)]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
enum HandType {
	HighCard = 0,
	OnePair = 1,
	TwoPair = 2,
	ThreeOfAKind = 3,
	FullHouse = 4,
	FourOfAKind = 5,
	FiveOfAKind = 6,
}