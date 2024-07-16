pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut curr = 0;
        let mut res = 0;

        for i in nums {
            match i {
                1 => {
                    curr+= 1;
                    if curr > res {
                    
                      res = curr

                    }

                    
                    
                },

                _ => curr = 0,

            }
           
        }
        res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let nums = vec![1, 1, 0, 1, 1, 1];
        let result = find_max_consecutive_ones(nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_all_ones() {
        let nums = vec![1, 1, 1, 1, 1];
        let result = find_max_consecutive_ones(nums);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_all_zeros() {
        let nums = vec![0, 0, 0, 0];
        let result = find_max_consecutive_ones(nums);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_mixed() {
        let nums = vec![1, 0, 1, 1, 0, 1];
        let result = find_max_consecutive_ones(nums);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_single_one() {
        let nums = vec![1];
        let result = find_max_consecutive_ones(nums);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_single_zero() {
        let nums = vec![0];
        let result = find_max_consecutive_ones(nums);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_empty() {
        let nums: Vec<i32> = vec![];
        let result = find_max_consecutive_ones(nums);
        assert_eq!(result, 0);
    }
}

fn main() {
  
  let nums = vec![1,1,0,1,1,1];

  let res = find_max_consecutive_ones(nums);

  println!("{:?}" , res);

}
