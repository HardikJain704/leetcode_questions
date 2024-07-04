pub fn multiply(num1: String, num2: String) -> String {

    if num1 == "0" || num2 == "0" {
        
        return "0".to_string();
    }

    let num1_as_bytes = num1.as_bytes();
    let num2_as_bytes = num2.as_bytes();
    let mut res = vec![0 ; num1.len() + num2.len()];


    for i in (0..num1.len()).rev() {
        for j in (0..num2.len()).rev() {
             
          let mul = (num1_as_bytes[i] - b'0') as usize  * ( num2_as_bytes[j] - b'0') as usize;
            let p1 = i+j;
            let p2 = i + j +1;
            let sum = mul + res[p2];

            res[p2] = sum % 10;
            res[p1] += sum / 10;
            

        }
    }


    let mut ans = String::new();
    let mut started = false;

    for &i in &res {
        if !(i == 0 && !started) {
            ans.push((i as u8 + b'0') as char);
            started = true;
        }
    }    

    ans


}


#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_multiply_with_zero(){
        let num1 = "21";
        let num2 = "0";

        let res = multiply(num1.to_owned(), num2.to_owned());

        assert_eq!(res, "0");

    }

    #[test]
    fn test_with_same_number(){
        let num1 = "12";
        let num2 = "12";
        let res = multiply(num1.to_owned(), num2.to_owned());
        assert_eq!(res , "144");

    }

    #[test]
    fn test_with_large_number(){

        let num1 = "123456789";
        let num2 = "123456789";
        let res = multiply(num1.to_owned(), num2.to_owned());
       
        assert_eq!(res, "15241578750190521")
    }
}




fn main() {
    let num1 = "2";
    let num2 = "3";
    let res = multiply(num1.to_owned(), num2.to_owned());

    println!("{:?}", res);

}
