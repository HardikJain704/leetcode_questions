pub fn min_window(s: String, t: String) -> String {
    if s.is_empty() || t.is_empty() || s.len() < t.len() {
        return String::new();
    }

    let mut target = [0; 128];
    let mut window = [0; 128];
    let c2idx = |c: char| c as usize;

    for c in t.chars() {
        target[c2idx(c)] += 1;
    }

    let mut left = 0;
    let mut right = 0;
    let mut count = 0;
    let mut min_left = 0;
    let mut min_len = usize::MAX;

    while right < s.len() {
        let c = s[right..=right].chars().next().unwrap();
        let idx = c2idx(c);
        window[idx] += 1;

        if window[idx] <= target[idx] {
            count += 1;
        }

        while count == t.len() {
            if right - left + 1 < min_len {
                min_left = left;
                min_len = right - left + 1;
            }

            let c = s[left..=left].chars().next().unwrap();
            let idx = c2idx(c);
            window[idx] -= 1;

            if window[idx] < target[idx] {
                count -= 1;
            }

            left += 1;
        }

        right += 1;
    }

    if min_len == usize::MAX {
        String::new()
    } else {
        s[min_left..min_left + min_len].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_case() {
        let s = "ADOBECODEBANC".to_string();
        let t = "ABC".to_string();
        let expected = "BANC".to_string();
        assert_eq!(min_window(s, t), expected);
    }

    #[test]
    fn test_no_substring_found() {
        let s = "ADOBECODEBANC".to_string();
        let t = "XYZ".to_string();
        let expected = "".to_string();
        assert_eq!(min_window(s, t), expected);
    }

    #[test]
    fn test_exact_match() {
        let s = "AAABBBCCC".to_string();
        let t = "AAABBBCCC".to_string();
        let expected = "AAABBBCCC".to_string();
        assert_eq!(min_window(s, t), expected);
    }

    #[test]
    fn test_single_character() {
        let s = "AAABBBCCC".to_string();
        let t = "A".to_string();
        let expected = "A".to_string();
        assert_eq!(min_window(s, t), expected);
    }


    #[test]
    fn test_t_longer_than_s() {
        let s = "A".to_string();
        let t = "AA".to_string();
        let expected = "".to_string();
        assert_eq!(min_window(s, t), expected);
    }

    #[test]
    fn test_empty_strings() {
        let s = "".to_string();
        let t = "".to_string();
        let expected = "".to_string();
        assert_eq!(min_window(s, t), expected);
    }

    #[test]
    fn test_empty_t() {
        let s = "ADOBECODEBANC".to_string();
        let t = "".to_string();
        let expected = "".to_string();
        assert_eq!(min_window(s, t), expected);
    }

    #[test]
    fn test_repeated_characters_in_t() {
        let s = "AAABBBCCC".to_string();
        let t = "AAA".to_string();
        let expected = "AAA".to_string();
        assert_eq!(min_window(s, t), expected);
    }
}

fn main() {
    let s = "ADOBECODEBANC";
    let t = "ABC";
    let res = min_window(s.to_owned(), t.to_owned());

    println!("{:?}", res);
}
