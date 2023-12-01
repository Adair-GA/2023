use std::{fs::File, io::Read};

fn parse_line(s: String) -> u32{
    let c: Vec<char> = s.chars().filter(|c| c.is_numeric()).collect();
    let str = c.get(0).unwrap().to_string() + &c.last().unwrap().to_string();
    str.parse::<u32>().unwrap()
}

fn read_file_to_vec(path: &str) -> Vec<String>{
    let mut f = File::open(path).unwrap();
    let mut s = String::new();

    f.read_to_string(&mut s).unwrap();
    s.split('\n').into_iter().map(|s| s.to_owned()).collect()
}

fn main(){
    let s: u32 = read_file_to_vec("data.txt").iter().map(|f| parse_line(f.to_string())).sum();
    println!("{}", s)
}


#[cfg(test)]
mod tests {
    use super::*;

    fn get_vec() -> Vec<String>{
        let v = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        v.into_iter().map(|s| s.to_owned()).collect()
    }   

    #[test]
    fn test_puntuacion() {
        let s: u32 = get_vec().iter().map(|f| parse_line(f.to_string())).sum();
        assert_eq!(s, 142)
    }
}