use std::collections::BTreeSet;

use crate::custom_error::AocError;

#[derive(Debug)]
struct Card{
    winning:	BTreeSet<u32>,
    yours:		BTreeSet<u32>
}

fn get_numbers(n_set: &str) -> BTreeSet<u32>{
	n_set.split(" ").filter(|n| *n != "").map(|n| n.parse::<u32>().unwrap()).collect::<BTreeSet<_>>()
}

impl Card{
	fn get_punctuation(&self) -> u32{
		let cuantity: u32 = self.winning.intersection(&self.yours).count().try_into().unwrap();
		if cuantity>0 {
			2_u32.pow(cuantity - 1)
		}else {
			0
		}
	}
}

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
	let cards = _input.lines()
	.map(|l| l.split(": ").skip(1).next().unwrap()).collect::<Vec<_>>();

	let result: u32 = cards.iter().map(|l| {
		let numbers = l.split(" | ").collect::<Vec<_>>();
		Card{
			winning: get_numbers(numbers.get(0).unwrap()),
			yours: get_numbers(numbers.get(1).unwrap())
		}
	}).map(|c| c.get_punctuation()).sum();

	Ok(result.to_string())
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
        assert_eq!("13", process(input)?);
        Ok(())
    }
}
