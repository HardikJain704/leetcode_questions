pub fn is_anagram(s: String, t: String) -> bool {
        let mut char1: Vec<char> = s.chars().collect();
        let mut char2: Vec<char>  = t.chars().collect();

        char1.sort();
        char2.sort();

        if char1 != char2{
            return false
        } 

        true

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagrams() {
        assert_eq!(is_anagram("listen".to_string(), "silent".to_string()), true);
        assert_eq!(is_anagram("triangle".to_string(), "integral".to_string()), true);
        assert_eq!(is_anagram("apple".to_string(), "papel".to_string()), true);
    }

    #[test]
    fn test_not_anagrams() {
        assert_eq!(is_anagram("hello".to_string(), "world".to_string()), false);
        assert_eq!(is_anagram("rust".to_string(), "crust".to_string()), false);
        assert_eq!(is_anagram("abcd".to_string(), "abcde".to_string()), false);
    }

    #[test]
    fn test_empty_strings() {
        assert_eq!(is_anagram("".to_string(), "".to_string()), true);
    }

    #[test]
    fn test_different_lengths() {
        assert_eq!(is_anagram("a".to_string(), "ab".to_string()), false);
    }
}

fn main() {
    let s = "hardik".to_string();
    let t = "kartik".to_string();
    let res = is_anagram(s, t);

    println!("{:?}" , res);

}
