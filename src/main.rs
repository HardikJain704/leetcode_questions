pub fn add_digits(mut num: i32) -> i32 {

    match num {
        0 => return 0,

        _ => {

            num = num % 9;
 
            if num == 0 {
                return 9
            }
            else{
                return num
            }

        }
    }
    



}


fn main() {
    let num = 121;
    let res = add_digits(num);

    println!("{:?}" , res);

}
