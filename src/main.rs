pub fn max_product(nums: Vec<i32>) -> i32 {

      if nums.is_empty() {
        return 0;
      } 

      let mut min_product : i32 = nums[0]; 
      let mut max_product : i32 = nums[0];
      let mut ans: i32 = nums[0];

      for value in nums.iter().skip(1){
        let max_product_possible =  max_product * value;
        let min_product_possible = min_product * value;

        max_product = min_product_possible.max(max_product_possible).max(*value);

        min_product = max_product_possible.min(min_product_possible).min(*value);

        ans = ans.max(max_product)
      }  

      ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_product1() {
        let nums = vec![2, 3, -2, 4];
        assert_eq!(max_product(nums), 6);
    }

    #[test]
    fn test_max_product2() {
        let nums = vec![-2, 0, -1];
        assert_eq!(max_product(nums), 0);
    }

    #[test]
    fn test_max_product3() {
        let nums = vec![2, 3, -2, 4, -1];
        assert_eq!(max_product(nums), 48);
    }

    #[test]
    fn test_max_product4() {
        let nums = vec![-2, -3, -4];
        assert_eq!(max_product(nums), 12);
    }

    #[test]
    fn test_max_product5() {
        let nums = vec![0, 2];
        assert_eq!(max_product(nums), 2);
    }

    #[test]
    fn test_max_product6() {
        let nums = vec![3, -1, 4];
        assert_eq!(max_product(nums), 4);
    }

    #[test]
    fn test_max_product7() {
        let nums = vec![-1, -3, -10, 0, 60];
        assert_eq!(max_product(nums), 60);
    }

    #[test]
    fn test_max_product8() {
        let nums = vec![6, -3, -10, 0, 2];
        assert_eq!(max_product(nums), 180);
    }

    #[test]
    fn test_max_product9() {
        let nums = vec![2, 3, -2, -4];
        assert_eq!(max_product(nums), 48);
    }

    #[test]
    fn test_max_product10() {
        let nums = vec![1];
        assert_eq!(max_product(nums), 1);
    }

    #[test]
    fn test_max_product_empty() {
        let nums: Vec<i32> = vec![];
        assert_eq!(max_product(nums), 0);
    }
}

fn main() {

 let nums = vec![2,3,-2,4];
 let res = max_product(nums);

 println!("{:?}" , res);

}
