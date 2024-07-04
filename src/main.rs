pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {

       if nums.is_empty() {
        return 0;
       }

       let mut count = 0;
        for i in 0..nums.len(){

         if  i < nums.len()-1 && nums[i] != nums[i+1] {
                 count += 1;
                 nums[count] = nums[i+1];
            }
            

            }
            (count + 1) as i32
        } 
       
    #[cfg(test)]

    mod tests{
        use super::*;

        #[test]
        fn test_remove_duplicates() {
            let mut test_cases = vec![
                (vec![1, 1, 2], vec![1, 2]),
                (vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4], vec![0, 1, 2, 3, 4]),
                (vec![], vec![]),
                (vec![1, 2, 3], vec![1, 2, 3]),
            ];
            for (input, expected) in &mut test_cases {
                let len = remove_duplicates(input);
                input.truncate(len as usize);
                assert_eq!(input, expected);
            }
        }
    }

fn main() {
        let mut nums = vec![1, 1, 2];
        let len = remove_duplicates(&mut nums);
        nums.truncate(len as usize);
        println!("{:?}", nums);
    }

