pub fn third_max(nums: Vec<i32>) -> i32 {
      let mut first_high = None;
      let mut second_high = None;
      let mut third_high = None;
      let mut tmp;

      for i in nums.iter() {

        let i = Some(i);

        if i > first_high {
            tmp = second_high;
            second_high = first_high;
            first_high = i;
            third_high = tmp;
        
        } else if i < first_high && i > second_high {
            tmp = second_high;
            second_high = i;
            third_high = tmp;
        }
        else if i < second_high && i > third_high {
            third_high = i;
        }
      }
      return *third_high.unwrap_or(first_high.unwrap());
        

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_third_max() {
        assert_eq!(third_max(vec![3, 2, 1]), 1);
        assert_eq!(third_max(vec![1, 2]), 2);
        assert_eq!(third_max(vec![2, 2, 3, 1]), 1);
        assert_eq!(third_max(vec![1, 2, 2, 5, 3, 5]), 2);
        assert_eq!(third_max(vec![1, 1, 1]), 1);
        assert_eq!(third_max(vec![2, 2, 3, 1, 4]), 2);
        assert_eq!(third_max(vec![5, 2, 4, 1, 3, 6, 0]), 4);
    }
}

fn main() {
    let nums = vec![3,2,1];

    let res = third_max(nums);

    println!("{:?}" , res);

}
