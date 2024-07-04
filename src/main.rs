pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0;
        let mut high = nums.len() -1;

        while low <= high {
            let mid = low + (high - low)/2;

            if nums[mid] == target {
                return mid.try_into().unwrap();
            }
            if nums[low] <= nums[mid] {
                if nums[low] <= target && target < nums[mid] {
                    high = mid-1;
                }else{
                    low = mid + 1;
                }
             }else{
                if nums[mid] < target && target <= nums[high]{
                    low = mid + 1;
                }
                else{
                    high = mid - 1;
                }
             }
        }
        -1
        
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]

    fn test_with_number_not_present_in_array(){
        let nums = [4,5,6,7,0,1,2];
        let target = 8;
        let res = search(nums.to_vec(), target);
        assert_eq!(-1, res);

    }

    #[test]

    fn test_with_single_element_in_array_and_not_present_in_array(){
        let nums = [1];
        let target = 8;
        let res = search(nums.to_vec(), target);
        assert_eq!(-1, res);

    }

    #[test]
    fn test_with_normal_array(){
        let nums = [4,5,6,7,0,1,2];
        let target = 0;
        let res = search(nums.to_vec(), target);
        assert_eq!(4, res);
        
    }
}
fn main() {
    let nums = [4,5,6,7,0,1,2];
    let target = 8;
    let res = search(nums.to_vec(), target);

    println!("{:?}" , res);
    
}
