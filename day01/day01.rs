// https://adventofcode.com/2023/day/1

use std::char::from_digit;
use std::fs::read_to_string;

fn part_one() -> u32 {
    let mut sum: u32 = 0;
    for line in read_to_string("input.txt").unwrap().lines() {
        let v = vec![
            line.chars().nth(line.find(|c: char| c.is_digit(10)).unwrap()).unwrap(),
            line.chars().nth(line.rfind(|c: char| c.is_digit(10)).unwrap()).unwrap()
        ];
        let num: u32 = v.into_iter().collect::<String>().parse().unwrap();
        sum += num;
    }
    sum
}

fn find_digit_by_name(s: String, reverse: bool) -> (Option<usize>, Option<char>) {
    let digit_names = vec![
        "one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine"
    ];
    let mut res_pos: Option<usize> = None;
    let mut res_digit: Option<char> = None;
    let mut i = 0;
    while i < digit_names.len() {
        let find_res = if reverse { s.rfind(digit_names[i]) } else { s.find(digit_names[i]) };
        if find_res.is_some() {
            let find_pos = find_res.unwrap();
            if res_pos.is_none() || (reverse == false && find_pos < res_pos.unwrap()) || (reverse == true && find_pos > res_pos.unwrap()) {
                res_pos = Some(find_pos);
                res_digit = Some(from_digit((i + 1) as u32, 10).unwrap());
            }
        }
        i += 1;
    }
    (res_pos, res_digit)
}

#[cfg(test)]
mod tests {
    use find_digit_by_name;
    #[test]
    fn test_find_digit_by_name() {
        assert_eq!(find_digit_by_name("two1nine".to_string(), false), (Some(0), Some('2')));
        assert_eq!(find_digit_by_name("two1nine".to_string(), true), (Some(4), Some('9')));
        assert_eq!(find_digit_by_name("eightwothree".to_string(), false), (Some(0), Some('8')));
        assert_eq!(find_digit_by_name("eightwothree".to_string(), true), (Some(7), Some('3')));
        assert_eq!(find_digit_by_name("abcone2threexyz".to_string(), false), (Some(3), Some('1')));
        assert_eq!(find_digit_by_name("abcone2threexyz".to_string(), true), (Some(7), Some('3')));
        assert_eq!(find_digit_by_name("xtwone3four".to_string(), false), (Some(1), Some('2')));
        assert_eq!(find_digit_by_name("xtwone3four".to_string(), true), (Some(7), Some('4')));
        assert_eq!(find_digit_by_name("kdzrjbh2txzz5hbone96one".to_string(), true), (Some(20), Some('1')));
    }
}

fn part_two() -> u32 {
    let mut sum: u32 = 0;

    for line in read_to_string("input.txt").unwrap().lines() {
        let first_by_digit = line.find(|c: char| c.is_digit(10));
        let last_by_digit = line.rfind(|c: char| c.is_digit(10));
        let first_by_name = find_digit_by_name(line.to_string(), false);
        let last_by_name = find_digit_by_name(line.to_string(), true);
        let first: char;
        let last: char;
        if first_by_digit.unwrap_or(usize::MAX) < first_by_name.0.unwrap_or(usize::MAX) {
            first = line.chars().nth(first_by_digit.unwrap()).unwrap();
        }
        else {
            first = first_by_name.1.unwrap();
        }
        if last_by_digit.unwrap_or(usize::MIN) >= last_by_name.0.unwrap_or(usize::MIN) {
            last = line.chars().nth(last_by_digit.unwrap()).unwrap();
        }
        else {
            last = last_by_name.1.unwrap();
        }
        let v = vec![ first, last ];
        let num: u32 = v.into_iter().collect::<String>().parse().unwrap();
        sum += num;
    }

    sum
}

fn main() {
    println!("--- Day 1: Trebuchet?! ---");
    println!("Part One: {}", part_one());
    println!("Part Two: {}", part_two());
}
