pub fn fib(n: i32) -> i32 {
      if n < 2 {
        return n
      } else {     
        return fib(n-1) + fib(n-2);
}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_0() {
        assert_eq!(fib(0), 0);
    }

    #[test]
    fn test_fib_1() {
        assert_eq!(fib(1), 1);
    }

    #[test]
    fn test_fib_2() {
        assert_eq!(fib(2), 1);
    }

    #[test]
    fn test_fib_3() {
        assert_eq!(fib(3), 2);
    }

    #[test]
    fn test_fib_4() {
        assert_eq!(fib(4), 3);
    }

    #[test]
    fn test_fib_5() {
        assert_eq!(fib(5), 5);
    }
}

fn main() {
    let n = 4;
    let res = fib(n);

    println!("{:?}" , res);

}
