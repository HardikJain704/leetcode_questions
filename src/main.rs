// In this what I am trying to do is 
//

use std::{collections::HashSet, ptr::null};
pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {

       if nums.len() < 3 {
        return vec![];
       } 

       nums.sort();

       let mut res = HashSet::new();

       for i in 0..nums.len() - 2 {
        let mut left_ptr = i+1;
        let mut right_ptr = nums.len() - 1;

        while left_ptr < right_ptr {
            let sum = nums[i] + nums[left_ptr] + nums[right_ptr];

            if sum == 0 {
                res.insert(vec![nums[i] , nums[left_ptr] , nums[right_ptr]]);
                left_ptr += 1;
                right_ptr -= 1;


            } else if sum < 0 {
                left_ptr += 1;
            }
            else{
                right_ptr -= 1;

            }
        }
       }
      
     let mut result_vec: Vec<Vec<i32>> =  res.into_iter().collect();
     result_vec.sort();
     result_vec

}


#[cfg(test)]

mod tests {
    use super::*;

    #[test]

    fn test_three_sum (){
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let result = three_sum(nums);
        let expected = vec![vec![-1, -1, 2] , vec![-1, 0, 1]];

        assert_eq!(expected , result);
    }

    #[test]
    fn test_result_not_possible(){
        let nums = vec![1, 2, -2, -1];
        let result = three_sum(nums);
        let expected: Vec<Vec<i32>> =  vec![];

        assert_eq!(expected , result);

    }

     #[test]

     fn test_with_large_numbers(){
        let nums = vec![1000000, -1000000, 0, 0, 0, 0];
        let result = three_sum(nums);
        let expected_res = vec![vec![-1000000, 0, 1000000], vec![0, 0, 0]];

        assert_eq!(expected_res , result);

     }

     
}

fn main() {
    let nums = vec![-1,0,1,2,-1,-4];
    let res = three_sum(nums);
    println!("{:?}" , res);

}
