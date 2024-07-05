pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len()-1;

        while left <= right {

        let mid = left + (right - left) / 2;

        if nums[mid]  == target {
            return mid.try_into().unwrap();
        }
            if nums[mid as usize] <  target {
                left = mid + 1;
            }
            else {
                 
                right = mid - 1;
                 
            }
        }
        left.try_into().unwrap()

}



#[cfg(test)]

mod test {
    use super::*;

    #[test]

    fn test_if_target_no_present_in_nums(){
        let nums = [1,3,5,6];
        let target = 2;

        let res = search_insert(nums.to_vec(), target);

        assert_eq!(1, res);

    }

    #[test]
    fn test_if_target_no_present_and_greater_then_largest_number_in_nums(){
        let nums = [1,3,5,6];
        let target = 7;

        let res = search_insert(nums.to_vec(), target);

        assert_eq!(4, res);
        
    }

}

fn main() {
   let nums = [1,3,5,6];
   let target = 5;

   let res = search_insert(nums.to_vec(), target);

   println!("{:?}" , res);
   
}
