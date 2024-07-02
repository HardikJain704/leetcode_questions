// In this first I created a variable then traversed over the vector and then checked if value doesnot matches with element inside nums vector 
// then assigning index of k to nums index and start again from next element and at end return count of k.
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut k = 0;

      for i in 0..nums.len() {
        if nums[i] != val {
            nums[k] = nums[i];
            k+=1;
        }
      }
      k as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_removal_of_element(){
        let mut nums = vec![0,1,2,2,3,0,4,2];
        let val = 2;
        let ans = remove_element(&mut nums, val);
        assert_eq!(ans, 5);
        assert_eq!(&nums[0..ans as usize], &[0, 1, 3, 0, 4]);
    }
    #[test]
    fn test_remove_element_empty() {
        let mut nums = vec![];
        let val = 1;
        let result = remove_element(&mut nums, val);
        assert_eq!(result, 0);
        assert_eq!(&nums[0..result as usize], &[]);
    }
    #[test]
    fn test_remove_element_repeating_element(){
        let mut nums = vec![4, 2, 4, 2, 4, 2];
        let val = 2;
        let result = remove_element(&mut nums, val);
        assert_eq!(result, 3);
        assert_eq!(&nums[0..result as usize], &[4, 4, 4]);
    }

    #[test]
    fn test_no_removal_of_element(){
        let mut nums = vec![1, 3, 5, 7];
        let val = 2;
        let result = remove_element(&mut nums, val);
        assert_eq!(result, 4);
        assert_eq!(&nums[0..result as usize], &[1, 3, 5, 7]);
    }
}

fn main() {
    let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let val = 2;
    let ans = remove_element(&mut nums, val);
    println!("Resulting length: {:?}", ans);
    println!("Modified array: {:?}", &nums[0..ans as usize]);
}