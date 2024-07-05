use std::{cmp::max, collections::HashMap};

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        
    
       let mut hashmap: HashMap<i32,bool> = HashMap::new();

       for &i in &nums {
        hashmap.insert(i , false);

       }

      
            let mut max_len = 0;

            for &i in &nums {
                if !hashmap[&i]{
            
            let mut current_num = i;  
            let mut current_len = 1;


            hashmap.insert(current_num, true);
            while hashmap.contains_key(&(current_num + 1)){

                current_num += 1;
                current_len += 1;
                hashmap.insert(current_num, true);

            }
            
            current_num = i;
            while hashmap.contains_key(&(&current_num - 1)){
                current_num -= 1;
                current_len += 1;
                hashmap.insert(current_num, true);


            }
            max_len = max_len.max(current_len);

        }

    }
    max_len
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_with_empty_vec(){
        let nums = vec![];
        let res = longest_consecutive(nums);

        assert_eq!(0 , res);
    }

    #[test]
    fn test_with_zero_to_ten(){
        let nums = vec![0,1,2,3,4,5,6,7,8,9,10];
        let res = longest_consecutive(nums);

        assert_eq!(11,res);

    }
}
fn main() {
       
       let nums = [100 , 4 ,200, 1 , 3 , 2];

       let res = longest_consecutive(nums.to_vec());

       assert_eq!(4, res);

       

}
