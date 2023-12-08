use std::{collections::HashMap, iter::zip, fmt::Error};

use nom::{IResult, bytes::complete::{tag, take_till}, multi::{separated_list0, many1}, character::{complete::{space1, u32, multispace0, line_ending}, is_digit}, Err};

use crate::custom_error::AocError;

#[derive(Default, Debug)]
struct Map{
	dest: u32,
	source: u32,
	len: u32
}

impl Map {
	fn get(&self, source: &u32) -> Option<u32>{
		if *source < self.source || *source >= self.source + self.len{return None};
		return Some(self.dest + (*source - self.source));
	}
}
#[derive(Default, Debug)]
struct Maps {
	entries: Vec<Map>
}

impl Maps{
	fn add(&mut self, m: Map){
		self.entries.push(m);
	}

	fn get(&self, source: u32) -> u32{
		let v = self.entries.iter().map(|map| map.get(&source)).filter(|o| o.is_some()).nth(0);
		if v.is_none(){
			source
		}else {
			v.unwrap().unwrap()
		}
	}
}

fn create_line(destination: u32, source: u32, len: u32) -> Map{
	Map {dest: destination, source, len}
}

fn parse_seeds(input:&str) -> IResult<&str, Vec<u32>>{
	let (input, _) = tag("seeds: ")(input)?;
	separated_list0(space1, u32)(input)
}

fn parse_map(input:&str) -> IResult<&str, Maps> {
	if input.is_empty() {
		return Err(Err::Error(nom::error::make_error(input, nom::error::ErrorKind::Eof)))
	}
	let (input, _) = multispace0(input)?;
	let (input, _) = take_till(|c| is_digit(c as u8))(input)?;
	let (input, lines) = separated_list0(
		line_ending,
		separated_list0(space1, u32)
	)(input)?;
	let lines = lines.into_iter().filter(|v| !v.is_empty()).collect::<Vec<Vec<u32>>>();
	let mut map = Maps::default();
	lines.into_iter().for_each(
		|vec| {
			map.add(create_line(
					*vec.get(0).unwrap(),
					*vec.get(1).unwrap(),
					*vec.get(2).unwrap()))
		}
	);
	Ok((input,map))
}
		
fn parse_maps(input:&str) -> Result<Vec<Maps>, Error> {
	let (_, res) = many1(parse_map)(input).unwrap();
	Ok(res)
}


fn get_location(maps: &Vec<Maps>, seed: u32) -> u32{
	let mut prev = seed;
	for map in maps {
		prev = map.get(prev)
	}
	prev
}

#[tracing::instrument]
pub fn process(
	_input: &str,
) -> miette::Result<String, AocError> {
	let (input, seeds) = parse_seeds(_input).unwrap();
	let maps = parse_maps(input).unwrap();
	let location = seeds.iter().map(|s| get_location(&maps, *s)).min().unwrap();
	return Ok(location.to_string());
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_process() -> miette::Result<()> {
		let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";	
		assert_eq!("35", process(input)?);
		Ok(())
	}
	
	#[test]
	fn map_creation() {
		let mut m = Maps::default();
		m.add(create_line(50, 98, 2));
		m.add(create_line(52, 50, 48));
		assert_eq!(m.get(98), 50_u32);
		assert_eq!(m.get(97), 99_u32);
		assert!(m.get(48) == 48)
	}

	#[test]
	fn seed_parse() {
		let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";	
		let (_, seeds) = parse_seeds(input).unwrap();
		assert_eq!(seeds.len(), 4);
		assert_eq!(seeds.get(0).unwrap(), &79_u32);
		assert_eq!(seeds.get(3).unwrap(), &13_u32);

	}
}
