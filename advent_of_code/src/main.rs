use std::fs;
use advent_of_code as lib;

fn main() {
    let contents = fs::read_to_string("./src/day_one_input.txt").unwrap();
    println!("{}", day_one(contents));
}

fn day_one(input: String) -> u32 {
    let mut total = 0;
    let mut new_input = input.clone();
    let mut offset = 0;
    for (old, new) in [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ] {
        for (i, _j) in input.match_indices(old) {
            new_input.insert(i+offset, new);
            offset+=1
        }
    }
    let input = new_input;
    println!("{input}");
    /*
    .replace("one", "1")
    .replace("two", "2")
    .replace("three", "3")
    .replace("four", "4")
    .replace("five", "5")
    .replace("six", "6")
    .replace("seven", "7")
    .replace("eight", "8")
    .replace("nine", "9");
    */
    for line in input.lines() {
        let mut first: Option<char> = None;
        let mut last: Option<char> = None;
        for ch in line.chars() {
            if ch.is_numeric() {
                if let None = first {
                    first = Some(ch);
                }
                last = Some(ch);
            }
        }
        if first.is_none() {
            println!("Continued");
            continue;
        };
        let msg = "The input line should always have a nuber";
        let mut first_and_last: String = first.expect(msg).to_string();
        first_and_last.push(last.expect(msg));
        let first_and_last = first_and_last.parse::<u32>().unwrap();
        println!("{first_and_last}");
        total += first_and_last;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::day_one;

    #[test]
    fn test_day_one() {
        let input = "1abc2\n\
                     pqr3stu8vwx\n\
                     a1b2c3d4e5f\n\
                     treb7uchet";
        let expected = 142;
        let result = day_one(input.to_string());
        assert_eq!(result, expected);
    }
    #[test]
    fn test_day_one_part_two() {
        let input = "two1nine\n\
                     eightwothree\n\
                     abcone2threexyz\n\
                     xtwone3four\n\
                     4nineeightseven2\n\
                     zoneight234\n\
                     7pqrstsixteen";
        let expected = 281;
        let result = day_one(input.to_string());
        assert_eq!(result, expected);
    }
}
