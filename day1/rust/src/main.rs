use std::{collections::HashMap, fs::File, io::BufRead, io::BufReader};

fn main() {
    let file = File::open("../input").unwrap();
    let lines = BufReader::new(file).lines();
    let mut sum: u32 = 0;
    for line in lines {
        if let Ok(l) = line {
            sum += find_config_value(&l);
        }
    }
    println!("{}", sum);
}

fn find_config_value(line: &str) -> u32 {
    let map = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("0", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("zero", 0),
    ]);

    let first = map
        .iter()
        .map(|(digit_string, value)| (line.find(digit_string), value))
        .filter(|(index, _)| index.is_some())
        .min()
        .unwrap()
        .1;

    let last = map
        .iter()
        .map(|(digit_string, value)| (line.rfind(digit_string), value))
        .max()
        .unwrap()
        .1;

    first * 10 + last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_config_value() {
        assert_eq!(find_config_value("nineight"), 98);
    }
}
