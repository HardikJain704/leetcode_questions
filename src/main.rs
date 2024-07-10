pub fn is_perfect_square(num: i32) -> bool {
        if num < 1  || num > 2147483646{
            return false;
        }

        let mut low = 1;
        let mut high = num;
        while low <= high {
            let mid = low + ( high - low ) / 2;
            let square = mid * mid;

            if square == num {
                return true;
            }

            else if square > num {
                high = mid - 1;
            }
            else {
                low = mid + 1;
            }
        }
        false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_perfect_square() {
        assert_eq!(is_perfect_square(16), true);
        assert_eq!(is_perfect_square(25), true);
        assert_eq!(is_perfect_square(14), false);
        assert_eq!(is_perfect_square(1), true);
        assert_eq!(is_perfect_square(0), false);
        assert_eq!(is_perfect_square(-4), false);
        assert_eq!(is_perfect_square(2147483647), false);
    }
}

fn main() {
    let num = 16;
    let res = is_perfect_square(num);
    println!("{:?}" , res);

}
