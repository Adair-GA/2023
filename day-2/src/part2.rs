use std::
str::FromStr;

use crate::custom_error::AocError;

const MAX_RED:      u32 = 12;
const MAX_GREEN:    u32 = 13;
const MAX_BLUE:     u32 = 14;


#[derive(Debug,Default)]
struct Set{
    red: u32,
    green: u32,
    blue: u32
}

#[derive(Default)]
struct Juego{
    id: u32,
    sets: Vec<Set>
}


#[derive(Debug)]
struct ParseLineError;

impl Set {
    fn is_valid(&self) -> bool{
        !(self.red > MAX_RED || self.blue > MAX_BLUE || self.green > MAX_GREEN)
    }

    fn mult(&self) -> u32{
        return self.red * self.blue * self.green;
    }
}

impl Juego{
    fn is_valid(&self) -> bool{
        self.sets.iter().all(|s| s.is_valid())
    }

    fn min_values(&self) -> u32{
        let mut min_set = Set::default();
        
        for set in &self.sets{
            if set.blue > min_set.blue{
                min_set.blue = set.blue;
            }
            if set.red > min_set.red{
                min_set.red = set.red;
            }
            if set.green > min_set.green{
                min_set.green = set.green;
            }
        }
        return min_set.mult();
    }
}

impl FromStr for Set{
    type Err = ParseLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set = Set::default();
        let colors = s.split(", ").collect::<Vec<_>>();
        for col in colors {
            let v = col.split(" ").collect::<Vec<_>>();
            let num = v.get(0).unwrap().parse::<u32>().unwrap();
            match *v.get(1).unwrap() {
                "red" => set.red += num,
                "blue" => set.blue += num,
                "green" => set.green += num,
                _ => return Err(ParseLineError)
            }
        }
        Ok(set)
    }
}

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let mut juegos: Vec<Juego> = Vec::new();
    for line in _input.lines(){
        let mut j = Juego::default();
        let game_line: Vec<&str> = line.split(": ").collect();
        let game_id = game_line.first().unwrap().chars().filter(|c| c.is_numeric()).collect::<String>().parse::<u32>().unwrap();
        let sets = game_line.get(1).unwrap().split("; ").collect::<Vec<_>>();
        j.id = game_id;
        for set in sets{
            j.sets.push(Set::from_str(set).unwrap())
        }
        juegos.push(j);
    }
    let suma_ids = juegos.iter().map(|j| j.min_values()).sum::<u32>().to_string();
    Ok(suma_ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("2286", process(input)?);
        Ok(())
    }

    #[test]
    fn set_from_str() {
        let i = "1 blue, 2 green, 3 red, 25 red";
        let s = Set::from_str(i).unwrap();
        assert_eq!(s.red, 28);
        assert_eq!(s.green, 2);
        assert_eq!(s.blue, 1);
        let i = "1 blue, 2 green, 3 red, 5 red";
        let s = Set::from_str(i).unwrap();
        assert_eq!(s.red, 8);
        assert_eq!(s.green, 2);
        assert_eq!(s.blue, 1);
        assert!(s.is_valid())
    }
}
