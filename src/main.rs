pub fn common_factors(a: i32, b: i32) -> i32 {
        if a == 1 && b == 1 {
            return 1;
        }

        let mut smaller: i32 = a.min(b);
        let mut count = 1;

        if a % b == 0 || b % a == 0 {
            count += 1;
        }
            for i in 2..=smaller/ 2 {
                if a % i == 0 && b % i == 0{
                    count += 1;
                }
               
            }
            count
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]

    fn test_with_two_numbers_with_only_one_common(){
        let a = 2;
        let b = 3;

        let res = common_factors(a, b);

        assert_eq!(1, res);

    
    }

    #[test]
    fn test_with_number_more_then_1000(){
        let a = 50;
        let b = 2000;

        let res = common_factors(a, b);

        assert_eq!(6, res);

    }
}


fn main() {
    let a = 12;
    let b = 6;

    let res = common_factors(a, b);

    println!("{:?}" , res);

}

