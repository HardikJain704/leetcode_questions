// In this I am using memosiazation in this case
// The T.C - O(n), S.C - O(n)

pub fn climb_stairs(n: i32) -> i32 {
    if n == 1 {
        return 1;

    }
    let mut dp = vec![0; (n+1) as usize];
        dp[1] = 1;
        dp[2] = 2;
        for i in 3..=n {
            dp[i as usize] = dp[i as usize-1] + dp[i as usize-2];

        }
    
    return dp[n as usize];


}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_for_len_4() {
    assert_eq!(climb_stairs(4) , 5);
    
  }

  #[test]
  fn test_for_len_5(){
    assert_eq!(climb_stairs(5) , 8);
  }
  

}
fn main() {
    let n: i32 = 3;
    let res = climb_stairs(n);
    println!("{:?}" , res);

}
