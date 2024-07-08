use std::collections::HashSet;

pub fn is_happy(mut n: i32) -> bool {
        let mut hash_set: HashSet<i32> = HashSet::new();

        while squaring(n) != 1 {
            n = squaring(n);

            if !hash_set.insert(n){
                return false;
            }
            
        }
        true
}


fn squaring(n: i32) -> i32 {

    n.to_string().chars().fold(0, |sum , c| {
        sum + c.to_digit(10).unwrap().pow(2)
    }) as i32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_happy() {
        assert_eq!(is_happy(1), true);  
        assert_eq!(is_happy(7), true);  
        assert_eq!(is_happy(19), true); 

        assert_eq!(is_happy(2), false); 
        assert_eq!(is_happy(3), false);  
        assert_eq!(is_happy(4), false); 
        assert_eq!(is_happy(18), false);
    }
}

fn main() {
  let n = 2;

  let res = is_happy(n);

  println!("{:?}" , res);

}
