use std::iter::zip;

use nom::{IResult, bytes::complete::take_till, character::{is_digit, complete::space1}, multi::separated_list0};

use crate::custom_error::AocError;

#[derive(Debug)]
struct Race{
    time: u64,
    record_distance: u64
}

fn parse_races(st: &str) -> IResult<&str, Vec<Race>>{
    let (input, _) = take_till(|chr| is_digit(chr as u8))(st)?;
    let (input, times) = separated_list0(space1, nom::character::complete::u64)(input)?;
    let (input, _) = take_till(|chr| is_digit(chr as u8))(input)?;
    let (input, records) = separated_list0(space1, nom::character::complete::u64)(input)?;
    let mut res = Vec::new();
    for (time, record_distance) in zip(times, records){
        res.push(
            Race{
                time,record_distance
            }
        )
    }
    Ok((input, res))
}


impl Race {
    fn get_times(&self) -> u64{
        let mut min  = 0u64;
        let mut max = 0u64;
        for press_time in 1..self.time{
            if press_time * (self.time - press_time) > self.record_distance{
                min = press_time;
                break;
            }
        }
        for press_time in (min..self.time).rev(){
            if press_time * (self.time - press_time) > self.record_distance{
                max = press_time;
                break;
            }
        }
        return max - min + 1
    }
}

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let (_, races) = parse_races(_input).unwrap();
    let times = races.iter()
    .map(|r| r.get_times())
    .fold(1u64, |acc, x| acc*x);
    Ok(times.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Time:      71530
Distance:  940200";
        assert_eq!("71503", process(input)?);
        Ok(()) 
    }

    #[test]
    fn test_parsing() {
        let input = "Time:      71530
Distance:  940200";
        let (_, res) = parse_races(input).unwrap();
        assert_eq!(res.len(), 1);
    }
}
