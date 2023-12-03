use itertools::Itertools;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let mut lines = _input.lines().peekable();
    let mut last: Option<&str> = None;
    let mut current = lines.next();
    let mut next = lines.next();
    let mut suma: u32 = 0;

    while current.is_some() {
        process_three_lines(last, current.unwrap(), next);
        last = current;
        current = next;
        next = lines.next();
    }

    todo!()
}

fn process_three_lines(last: Option<&str>, current: &str, next: Option<&str>) -> u32{
    let chars_act = current.chars().enumerate();
    let mut last_digit = 0;
    let mut suma_validos: u32 = 0;
    for (index, char) in chars_act{
        if last_digit > 0{
            last_digit-=1;
            continue;
        }
        if char.is_ascii_digit(){
            last_digit = 2;
            let mut found = false;
            if let Some(last_line) = last {
                found = surrounding_line_has_chars(index, last_line)
            }
            if let Some(next_line) = next {
                found = surrounding_line_has_chars(index, next_line)
            }
            if found {
                suma_validos += current.get(index..index+3).unwrap().parse::<u32>().unwrap();
            }
        }
    }
    return suma_validos;
}

fn surrounding_line_has_chars(index: usize, line: &str) -> bool{
    let start = if index == 0 {0} else {index-1};
    let end = if index + 3 > line.len() {line.len()} else {index + 3};
    let chars = line.get(start..=end).unwrap().chars().filter(|c| *c != '.').try_len().unwrap();
    chars>0
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
        assert_eq!("", process(input)?);
        Ok(())
    }
}
