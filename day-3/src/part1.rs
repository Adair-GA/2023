use std::process::Output;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let mut lines = _input.lines().peekable();
    let mut last: Option<&str> = None;
    let mut current = lines.next();
    let mut next = lines.next();
    let mut suma: Vec<u32> = Vec::new();

    while current.is_some() {
        suma.push(process_three_lines(last, current.unwrap(), next));
        last = current;
        current = next;
        next = lines.next();
    }
    dbg!(suma.iter().any(|f| *f > 9999));
    Ok(suma.iter().sum::<u32>().to_string())
}

fn process_three_lines(last: Option<&str>, current: &str, next: Option<&str>) -> u32{
    let chars_act = current.chars().enumerate();
    let chars_v =chars_act.clone().collect::<Vec<_>>();
    let mut last_digit = 0;
    let mut suma_validos: u32 = 0;
    for (index, char) in chars_act{
        if last_digit > 0{
            last_digit-=1;
            continue;
        }
        if char.is_ascii_digit(){
            let mut i = index;
            let mut num_of_digits = 0;
            while let Some((_, d)) = chars_v.get(i) {
                if !d.is_ascii_digit(){break;}
                num_of_digits+=1;
                i+=1;
            }
            let mut found = false;
            if let Some(last_line) = last {
                found = surrounding_line_has_chars(index, last_line, num_of_digits)
            }
            if !found{
                if let Some(next_line) = next {
                    found = surrounding_line_has_chars(index, next_line, num_of_digits)
                }
            }
            if !found{
                let anterior = if index != 0 {chars_v.get(index - 1)} else {Option::None};
                let posterior = chars_v.get(index+num_of_digits);
                if let Some((_, d)) = anterior {
                    if *d != '.' {found = true}
                }
                if let Some((_, d)) = posterior {
                    if *d != '.' {found = true}
                }
                
            }
            if found {
                suma_validos += current.get(index..index+num_of_digits).unwrap().parse::<u32>().unwrap();
            }
            last_digit = num_of_digits -1 ;
        }
    }
    return suma_validos;
}

fn surrounding_line_has_chars(index: usize, line: &str, num_of_digits: usize) -> bool{
    let start = if index == 0 {0} else {index-1};
    let end = if index + num_of_digits > (line.len() - 1) {line.len() - 1} else {index + num_of_digits};
    let char = line.get(start..=end).unwrap().chars().filter(|c| *c != '.').next();
    return char.is_some();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("4361", process(input)?);
        Ok(())
    }
}
