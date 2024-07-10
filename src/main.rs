pub fn is_ugly(mut n: i32) -> bool {

      if n <= 0 {
        return false;
      } 

      while n % 2 == 0 {
          n = n / 2;
       } 

       while  n % 3 == 0 {
        n = n / 3;

       } 
         while n % 5 == 0 {
            n = n / 5;
     
       } 

       if n == 1 {
        return true
       }
       
       return false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ugly() {
        assert_eq!(is_ugly(6), true);
        assert_eq!(is_ugly(8), true);
        assert_eq!(is_ugly(14), false);
        assert_eq!(is_ugly(1), true);
        assert_eq!(is_ugly(0), false);
        assert_eq!(is_ugly(-6), false);
    }
}

fn main() {
   let n = 14;

   let res = is_ugly(n);


   println!("{:?}" , res);

}
