pub fn longest_palindrome(s: String) -> String {
     if s.is_empty(){
        return s;
     }       

     let sv: Vec<char> = s.chars().collect();
     let len = sv.len();
 
     let mut start = 0;
     let mut end = 0;

     for i in 0..s.len() {

        let mut left = i;
        let mut right = i;
       
       while right + 1 < len && sv[right + 1] == sv[left]{
        right+=1;
       }

       while right + 1 < len && left > 0 && sv[right + 1] == sv[left - 1]{
        right += 1;
        left -= 1;
       } 

       if right - left > end - start {
          end = right;
          start = left;
       }

  
     }
     s[start..end+1].to_string()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_char_palindrome() {
     let string = "a";
     let res = longest_palindrome(string.to_owned());
     let expected_res = "a";
     assert_eq!(expected_res , res);
    }

    #[test]
    fn test_empty_string() {
     let string = "";
     let res = longest_palindrome(string.to_owned());
     let expected_res = "";
     assert_eq!(expected_res , res);
    }

    }

fn main() {
   
   let s = "cbbd";
    let res = longest_palindrome(s.to_owned());
    println!("{:?}" , res);
    
}
