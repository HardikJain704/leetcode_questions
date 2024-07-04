pub fn length_of_last_word(s: String) -> i32 {
     if s.len() == 0 {
        return 0;
     }

     let trimmed = s.trim_end();
     let mut count = 0;
     for i in trimmed.chars().rev() {
         if i != ' ' {
            count+=1;
         }
        
        else{
            break;
        }
     }
     count

}


#[cfg(test)]

mod tests {
    use super::*;

    #[test]

    fn test_length_of_word_empty(){

        assert_eq!(length_of_last_word("".to_string()), 0);
    }

    #[test]
    fn test_spaces_only(){
        assert_eq!(length_of_last_word("   ".to_string()) , 0);

    }

    #[test]
    fn test_single_char_only(){
        assert_eq!(length_of_last_word("H".to_string()) , 1);

    }
}
fn main() {
    let str = "   fly me   to   the moon  ";
    let res = length_of_last_word(str.to_owned());
    println!("{:?}" , res);
}
