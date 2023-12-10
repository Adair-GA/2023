use std::{collections::HashMap, iter::zip, cmp::Ordering};

use itertools::Itertools;

use crate::custom_error::AocError;

enum Type{
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

#[derive(Debug, Hash, Clone, Copy)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    Number(u8)
}

impl Card {
    fn new(c: char) -> Card{
        match c {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            _ => Card::Number(c.to_digit(10).unwrap() as u8),
        }
    }

    fn get_value(&self) -> u8{
        match self {
            Card::A => 14,
            Card::K => 13,
            Card::Q => 12,
            Card::T => 10,
            Card::Number(x) => *x,
            Card::J => 1
        }
    }
}

impl Eq for Card{}

impl PartialOrd for Card{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.get_value().cmp(&other.get_value()))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.get_value().eq(&other.get_value())
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand{
    cards: Vec<Card>,
    bid: u32
}

impl PartialOrd for Hand{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let ord = (self.get_type() as u8).cmp(&(other.get_type() as u8));
        match ord {
            std::cmp::Ordering::Equal => {
                for (cself, cother) in zip(self.cards.clone(), other.cards.clone()){
                    let o = cself.partial_cmp(&cother).unwrap();
                    if !o.is_eq(){
                        if o.is_gt() {return Some(Ordering::Less)} else {
                            return Some(Ordering::Greater);
                        }
                    }
                }
                return None;
            }
            _ => Some(ord)
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug)]
struct HandError;

impl Hand{
    fn new(cards: &str, bid: u32) -> Result<Hand, HandError>{
        if cards.len() != 5 {Err(HandError)}
        else {
            Ok(Hand { cards: cards.chars().map(|c| Card::new(c)).collect_vec(), bid})
        }
    }

    fn get_type(&self) -> Type{
        let mut quantities: HashMap<Card, u8> = self.cards.iter().fold(HashMap::new(), |mut acc, c|{
            acc.entry(c.clone()).and_modify(|i| *i+=1).or_insert(1);
            acc
        });
        if let Some(j) = quantities.clone().get(&Card::J) {
            if *j == 5 {return Type::FiveOfAKind;}
            let mut max = (Card::J, 0);
            quantities.iter().for_each(|(c, q)| {
                if !matches!(c, Card::J) && q > &max.1{
                    max = (*c, *q)
                }
            });
            if max.1 > 0{
                quantities.entry(max.0).and_modify(|v| *v+=j);
                quantities.remove(&Card::J);
            }
        }
        let mut set = quantities.values().collect::<Vec<&u8>>();
        set.sort();
        match set.as_slice() {
            [1,1,1,1,1] => Type::HighCard,
            [1,1,1,2] => Type::OnePair,
            [1,2,2] => Type::TwoPair,
            [1,1,3] => Type::ThreeOfAKind,
            [2,3] => Type::FullHouse,
            [1,4] => Type::FourOfAKind,
            [5] => Type::FiveOfAKind,
            _ => panic!()
        }
    }
}

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let mut hands = _input.lines().map(|l| l.split_ascii_whitespace())
    .map(|mut l| {
        Hand::new(l.nth(0).unwrap(), l.nth(0).unwrap().parse::<u32>().unwrap()).unwrap()
    }).collect_vec();
    hands.sort();
    let len = hands.len();
    let sum: usize = hands.into_iter().enumerate().map(|(i, h)| {
        let rank = len - i;
        h.bid as usize * rank
    }).sum();

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!("5905", process(input)?);
        Ok(())
    }
}
