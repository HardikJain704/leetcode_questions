pub fn repeated_substring_pattern(s: String) -> bool {
        let n = s.len();
        for i in 1..= n/2 {

        if n % i == 0 {
            let substring = &s[0..i];
            let mut repeated = String::new();
            for j in 0..(n / i) {
                repeated.push_str(substring);
            }
            if repeated == s {
                return true;
            }
           
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeated_substring_pattern() {
        assert_eq!(repeated_substring_pattern("abab".to_string()), true);
        assert_eq!(repeated_substring_pattern("aba".to_string()), false);
        assert_eq!(repeated_substring_pattern("abcabcabcabc".to_string()), true);
        assert_eq!(repeated_substring_pattern("a".to_string()), false);
        assert_eq!(repeated_substring_pattern("ababab".to_string()), true);
        assert_eq!(repeated_substring_pattern("abababab".to_string()), true);
        assert_eq!(repeated_substring_pattern("ababababa".to_string()), false);
    }

    #[test]
    fn test_repeated_substring_pattern_edge_cases() {
        assert_eq!(repeated_substring_pattern("".to_string()), false);
        assert_eq!(repeated_substring_pattern("aaaa".to_string()), true);
        assert_eq!(repeated_substring_pattern("xyzxyz".to_string()), true);
        assert_eq!(repeated_substring_pattern("abcdef".to_string()), false);
        assert_eq!(repeated_substring_pattern("aaaaa".to_string()), true);
    }
}

fn main() {
    let s = "abab";
    let res = repeated_substring_pattern(s.to_owned());

    println!("{:?}" , res);
    
}
