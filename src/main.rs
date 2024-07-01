pub fn is_palindrome(x: i32) -> bool {  
    if x < 0 {
        return false;
    }
    else {
        let mut reverse = 0;
        let mut number = x;
  
      while number != 0{
        reverse = reverse * 10 + number % 10;
        number = number / 10; 
        
    }
   return reverse == x;
   
}
}

fn main(){

    let result = is_palindrome(121);
    if result {
    println!("true");
    }else {
    println!("false");
}

}