pub fn fizz_buzz(n: i32) -> Vec<String> {

    let mut str: Vec<String> = Vec::new();

    for i in 1..= n {

     if i % 3 == 0 && i % 5 == 0 {
        str.push("FizzBuzz".to_string());
     }

     else if  i % 3 == 0  {

        str.push("Fizz".to_string());
         
     }

     else if i % 5 == 0 {
        str.push("Buzz".to_string());

     }
     else {

        str.push(i.to_string());

     }
    }
    str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizz_buzz() {
        assert_eq!(fizz_buzz(1), vec!["1"]);
        assert_eq!(fizz_buzz(3), vec!["1", "2", "Fizz"]);
        assert_eq!(fizz_buzz(5), vec!["1", "2", "Fizz", "4", "Buzz"]);
        assert_eq!(fizz_buzz(15), vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz",
            "11", "Fizz", "13", "14", "FizzBuzz"
        ]);
    }

    #[test]
    fn test_fizz_buzz_with_0() {
        // Testing with 0 should return an empty vector
        assert_eq!(fizz_buzz(0), Vec::<String>::new());
    }

    #[test]
    fn test_fizz_buzz_negative() {
        // Testing with negative numbers should return an empty vector
        assert_eq!(fizz_buzz(-1), Vec::<String>::new());
    }

}

fn main() {
    let n = 15;
    let res = fizz_buzz(n);

    println!("{:?}" , res);

}
