use std::collections::HashSet;


pub fn length_of_longest_substring(s: String) -> i32 {
        
        let mut longest = 0;
        for i in 0..s.len() {
            let mut hash: HashSet<char> = HashSet::new();
            for (j , c) in s.chars().skip(i).enumerate() {
                if !hash.contains(&c) {
                    hash.insert(c);
                    longest = longest.max(j + 1);
                }else{
                    break;
                }
                
            }


        }
        longest as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    
    fn test_string_with_same_chars(){
        let s = "hhhhh".to_owned();
        assert_eq!(1, length_of_longest_substring(s))
    }

    #[test]
    fn test_string_with_spaces_symbols (){
        let s = "H!ar#d&i*k".to_owned();
        assert_eq!(10, length_of_longest_substring(s));
    }

    #[test]
    fn test_with_random_string(){
        let s = "pwwkew".to_owned();
        assert_eq!(3, length_of_longest_substring(s));
    }
}

fn main() {
    let s = "abcabcbb";
    let res = length_of_longest_substring(s.to_owned());
    println!("{:?}" , res);
    
}
