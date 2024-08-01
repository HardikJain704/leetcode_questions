
pub fn is_happy(n: i32) -> bool {

    if n < 1 {
        return false;

    }

    let mut sum: i32 = n;
    let mut verifier: Vec<i32> = vec![];

    while sum != 1 {
        let digits: Vec<i32> = sum.to_string().chars().map(|dig| dig.to_digit(10).unwrap() as i32).collect();
        sum = digits.iter().map(|element| element.pow(2)).sum();

        if verifier.contains(&sum) {
            return false;
        } else {
            verifier.push(sum);
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_happy_number() {
        assert_eq!(is_happy(19), true);
        assert_eq!(is_happy(1), true);
        assert_eq!(is_happy(7), true);
    }

    #[test]
    fn test_unhappy_number() {
        assert_eq!(is_happy(2), false);
        assert_eq!(is_happy(4), false);
        assert_eq!(is_happy(20), false);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(is_happy(0), false);
        assert_eq!(is_happy(-1), false);
    }
}


fn main() {
    let n = 19;
    let res = is_happy(n);

    println!("{:?}" , res);

}
