pub fn remove_whitespace(s: &mut String) -> String {
    s.retain(|c| !c.is_whitespace());
    let x = s;
    x.to_string()
}

use regex::Regex;

pub fn remove_punctuation(text: &str) -> String {
    let re = Regex::new(r"<<|>>|<|>|«|»|,|'|\.|;|:").unwrap();
    // Replace all matches with an empty string

    (re.replace_all(text, "").as_ref())
        .trim()
        .to_uppercase()
        .to_string()
}

pub fn generate_keyword_string(keyword: &mut String, plaintext_len: usize) -> String {
    if plaintext_len > keyword.len() {
        let remainder = plaintext_len % keyword.len();
        let quotient = plaintext_len / keyword.len();
        let mut keyword_string = keyword.repeat(quotient).to_string();
        dbg!("{} -- {}", remainder, quotient);
        if remainder > 0 {
            keyword_string.push_str(&keyword[0..remainder]);
        }
        dbg!("{}, {}", &keyword_string, &keyword_string.len());
        keyword_string
    } else {
        keyword[0..plaintext_len].to_string()
    }
}

pub fn remove_repeating_letters(text: String) -> String {
    let mut letters = Vec::new();
    let mut new_string = text;
    new_string.retain(|x| {
        if x.is_whitespace() {
            return true;
        }
        println!("{}", x);
        if letters.contains(&x) {
            false
        } else {
            letters.push(x);
            true
        }
    });
    new_string
}

pub fn remove_charset(charset: Vec<char>, text: String) -> String {
    let mut result = text;
    result.retain(|x| !charset.contains(&x));
    result
}

pub fn find_char(grid: &Vec<Vec<char>>, target: char) -> Option<(usize, usize)> {
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, c) in row.iter().enumerate() {
            if *c == target {
                return Some((col_idx, row_idx));
            }
        }
    }

    None
}
pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_repeating() {
        let test_string = String::from("dammed to be here");
        assert_eq!("dame to b hr", remove_repeating_letters(test_string))
    }

    #[test]
    fn test_remove_charset() {
        let test_string = String::from("abcdef");
        assert_eq!(
            String::from("abcd").trim(),
            remove_charset(Vec::from(['e', 'f']), test_string).trim()
        )
    }

    #[test]
    fn test_finder() {
        let grid = Vec::from([Vec::from(['a', 'b']), Vec::from(['c', 'd'])]);
        assert_eq!(Some((0, 1)), find_char(&grid, 'c'))
    }
}
