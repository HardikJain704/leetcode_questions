pub fn shortest_palindrome(s: String) -> String {
        let reverse_string = s.chars().rev().collect::<String>();
  
         for i in 0..reverse_string.len() {
            if s.starts_with(&reverse_string[i..]) {
                return format!("{}{}",  &reverse_string[..i], s);

            }
           
         }


         format!("{}{}" , &reverse_string , s)

     
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_palindrome() {
        assert_eq!(shortest_palindrome("aacecaaa".to_string()), "aaacecaaa");
        assert_eq!(shortest_palindrome("abcd".to_string()), "dcbabcd");
        assert_eq!(shortest_palindrome("a".to_string()), "a");
        assert_eq!(shortest_palindrome("racecar".to_string()), "racecar");
        assert_eq!(shortest_palindrome("abac".to_string()), "cabac");
    }
}  

fn main() {
   let s = "abcd";

   let res = &shortest_palindrome(s.to_owned());

   println!("{:?}" , res);
   
}
 