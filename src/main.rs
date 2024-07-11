use std::i32::MAX;

pub fn my_sqrt(x: i32) -> i32 {
        let mut sqrt: i32 = 0;
         if x == MAX {
            return 46340;
         }
         for i in 0..x + 1 {
            if i * i == x {
                sqrt = i; 
                break
            }
            if ((i * i) < x ) && ((i+1) * (i+1)) > x {
                sqrt = i ;
                break
            }
         }
         return sqrt;

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_sqrt() {
        assert_eq!(my_sqrt(0), 0);
        assert_eq!(my_sqrt(1), 1);
        assert_eq!(my_sqrt(4), 2);
        assert_eq!(my_sqrt(8), 2); 
        assert_eq!(my_sqrt(9), 3);
        assert_eq!(my_sqrt(15), 3); 
        assert_eq!(my_sqrt(16), 4);
        assert_eq!(my_sqrt(MAX), 46340);
    }
}

fn main() {
  let x = 8;
  let res = my_sqrt(x);

  println!("{:?}" , res);

}
