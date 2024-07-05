pub fn reverse_words(s: String) -> String {
        let words_array: Vec<&str> = s.split(" ").collect();
        let mut res: Vec<&str> = vec![];
        for i in (0..words_array.len()).rev(){
            if words_array[i] != "" {
                res.push(words_array[i])
            }
        }
        res.join(" ")
}

#[cfg(test)]

mod test{
    use super::*;

    #[test]
    fn test_with_string_having_other_characters_also() {
        let str = "the sky! $ is bl^ue@";
        let res = reverse_words(str.to_owned());
        assert_eq!("bl^ue@ is $ sky! the" , res);

    }

    #[test]
    fn test_with_single_word(){
        let str = "hardik!";
        let res = reverse_words(str.to_owned());
        assert_eq!("hardik!" , res);
    
    }

    #[test]
    fn test_with_empty_string(){
        let str = " ";
        let res = reverse_words(str.to_owned());

        assert_eq!("" , res);


    }

}


fn main() {
    let str = "the sky! $ is blue!";
    let res = reverse_words(str.to_owned());
    println!("{:?}" , res);

}
