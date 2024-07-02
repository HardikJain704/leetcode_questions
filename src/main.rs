/// In this what I am trying to do is first created a variable and copied number to it.
/// then divided the number so that I can the exact digit at nth place 
/// and then pushed it and then traversing over it again and then multipling again to 
/// add its nth value and like that the number would be reversed. 
/// added some checked to suppose if I got something 320 and last digit over here is 0 which when comes to first place
/// make no sense so it should not be added over there so added a check for that as well.

 
pub fn reverse(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }

        let mut digits = vec![];
        let mut copy_digit = x;
        while copy_digit != 0 {
            digits.push(copy_digit % 10);
            copy_digit /= 10;
        }
        let mut res = 0;

        for i in digits {

            if res > i32::MAX / 10 || res == i32::MAX / 10 && i > i32::MAX % 10 {
                return 0;
            }
            if res < i32::MIN / 10 || res == i32::MIN / 10 && i < i32::MIN % 10 {
                return 0;
            }
            res = res * 10 + i;
        }
        return res;
}


#[cfg(test)]
mod tests {
  use super::*;
   
   #[test]
   fn test_reverse() {
       assert_eq!(reverse(123), 321);
   }

   #[test]
   fn test_reverse_negative() {
       assert_eq!(reverse(-123), -321);
   }
      #[test]
    fn test_reverse_having_zero() {
        assert_eq!(reverse(120), 21);
    }
}

fn main() {
    let no = 11113;
    let reverse = reverse(no);
    println!("{:?}" , reverse);

}
 