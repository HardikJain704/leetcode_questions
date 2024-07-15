pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
    let mut res = Vec::new();
    let mut curr = Vec::new();
    let mut num_of_letters: i32 = 0;

    for word in words {
        if (num_of_letters + word.len() as i32 + curr.len() as i32) > max_width {
            let spaces_needed = max_width - num_of_letters;
            let gaps = curr.len() as i32 - 1;
            if gaps > 0 {
                let spaces_per_gap = (spaces_needed / gaps) as usize;
                let extra_spaces = (spaces_needed % gaps) as usize;
                for i in 0..extra_spaces {
                    curr[i] = format!("{} ", curr[i]);
                }
                res.push(curr.join(&" ".repeat(spaces_per_gap)));
            } else {
                res.push(curr.join("") + &" ".repeat(spaces_needed as usize));
            }
            curr.clear();
            num_of_letters = 0;
        }
        curr.push(word.clone());
        num_of_letters += word.len() as i32;
    }

    // Handle the last line
    if !curr.is_empty() {
        let last_line = curr.join(" ");
        res.push(format!("{:<width$}", last_line, width = max_width as usize));
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_case() {
        let words: Vec<String> = vec!["This".to_string(), "is".to_string(), "an".to_string(), "example".to_string(), "of".to_string(), "text".to_string(), "justification".to_string()];
        let max_width = 16;
        let expected: Vec<String> = vec![
            "This    is    an".to_string(),
            "example  of text".to_string(),
            "justification   ".to_string()
        ];
        assert_eq!(full_justify(words, max_width), expected);
    }

    #[test]
    fn test_single_word() {
        let words: Vec<String> = vec!["singleword".to_string()];
        let max_width = 16;
        let expected: Vec<String> = vec!["singleword      ".to_string()];
        assert_eq!(full_justify(words, max_width), expected);
    }

    #[test]
    fn test_all_words_fit_in_one_line() {
        let words: Vec<String> = vec!["this".to_string(), "is".to_string(), "fine".to_string()];
        let max_width = 16;
        let expected: Vec<String> = vec!["this is fine    ".to_string()];
        assert_eq!(full_justify(words, max_width), expected);
    }

    #[test]
    fn test_empty_words() {
        let words: Vec<String> = vec![];
        let max_width = 10;
        let expected: Vec<String> = vec![];
        assert_eq!(full_justify(words, max_width), expected);
    }

    #[test]
    fn test_long_single_word() {
        let words: Vec<String> = vec!["supercalifragilisticexpialidocious".to_string()];
        let max_width = 34;
        let expected: Vec<String> = vec!["supercalifragilisticexpialidocious".to_string()];
        assert_eq!(full_justify(words, max_width), expected);
    }
}

fn main() {
    let words: Vec<String> = vec!["This".to_string(), "is".to_string(), "an".to_string(), "example".to_string(), "of".to_string(), "text".to_string(), "justification".to_string()];
    let max_width = 16;
    let res = full_justify(words, max_width);
    println!("{:?}", res);
}
