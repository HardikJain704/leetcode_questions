pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;

        for i in 0..nums.len() {
            while nums[i] > 0 && (nums[i] < nums.len() as i32) && nums[i] != i as i32 + 1 {
                let j = nums[i] as usize - 1;
                if nums[j] == nums[i] {
                    break
                }
                nums.swap(i , j);

            } 
        }

        for i in 0..nums.len(){
            if i as i32 + 1 != nums[i] {
                return i as i32 + 1
            }
        }
        nums.len() as i32 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_missing_positive() {
        assert_eq!(first_missing_positive(vec![1, 2, 0]), 3);
        assert_eq!(first_missing_positive(vec![3, 4, -1, 1]), 2);
        assert_eq!(first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
        assert_eq!(first_missing_positive(vec![]), 1);
        assert_eq!(first_missing_positive(vec![1, 2, 3, 4, 5]), 6);
        assert_eq!(first_missing_positive(vec![1, 1, 0, -1, -2]), 2);
    }
}

fn main() {
   let nums = vec![3,4,-1, 1];

   let res = first_missing_positive(nums);

   println!("{:?}" , res);

}
