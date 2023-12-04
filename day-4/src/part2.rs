use std::collections::{BTreeSet, HashMap};

use crate::custom_error::AocError;

#[derive(Debug, PartialEq,Eq, Hash, Clone)]
struct Card{
    winning:	BTreeSet<u32>,
    yours:		BTreeSet<u32>
}

fn get_numbers(n_set: &str) -> BTreeSet<u32>{
	n_set.split(" ").filter(|n| *n != "").map(|n| n.parse::<u32>().unwrap()).collect::<BTreeSet<_>>()
}

impl Card{
	fn get_puntuacion(&self) -> usize{
		self.winning.intersection(&self.yours).count()
    }
}

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let lines = _input.lines()
	.map(|l| l.split(": ").skip(1).next().unwrap()).collect::<Vec<_>>();

    let result = lines.iter().map(|l| {
        let numbers = l.split(" | ").collect::<Vec<_>>();
        Card{
            winning: get_numbers(numbers.get(0).unwrap()),
            yours: get_numbers(numbers.get(1).unwrap())
        }
    }).collect::<Vec<Card>>();

    let mut card_cuantities: HashMap<Card, u32> = result.iter().map(|c| (c.clone(), 1_u32)).collect();
    
    for (i, card) in result.iter().enumerate(){
        if i % 10 == 0 {
            dbg!(i);
        }
        let reps: usize = card.get_puntuacion();
        for add_to in i+1..=i+reps {
            if reps == 0 {break;}
            let card_to_repeat = result.get(add_to);
            if !card_to_repeat.is_some() {break;}
            let q = card_cuantities.get(card_to_repeat.unwrap()).unwrap() + *card_cuantities.get(card).unwrap();
            card_cuantities.insert(card_to_repeat.unwrap().clone(), q);
        }
    }


	Ok(card_cuantities.values().sum::<u32>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("30", process(input)?);
        Ok(())
    }
}
