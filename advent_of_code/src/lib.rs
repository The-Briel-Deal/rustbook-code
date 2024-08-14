pub fn slice_starts_with_number(slice: &str) -> Option<char> {
    for (word, num) in [
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
        if slice.starts_with(word) {
            return Some(num);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::slice_starts_with_number;

    #[test]
    fn test_slice_starts_with_number() {
        let tc = [
            ("abconetwothree", None),
            ("two", Some('2')),
            ("threeeeadsjflksd", Some('3')),
        ];
        for (input, expected) in tc {
            let result = slice_starts_with_number(input);
            assert_eq!(result, expected);
        }
    }
}
