fn main() {
    let s = "Boogie Woogie";
    let fws = first_word(s);

    println!("s = {s}, first word = {fws}")
}

fn first_word(s: &str) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
/*
 * fn get_space_index(s_bytes: &[u8]) -> usize {
 *     for (i, &item) in s_bytes.iter().enumerate() {
 *         let item = item as char;
 *         println!("i = {i}, item = {item}");
 *         if item == ' ' {
 *             return i;
 *         }
 *     }
 *
 *     s_bytes.len()
 * }
 */
