use std::collections::HashMap;
pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {   
        let mut count = HashMap::new();

        for &i in nums.iter(){
            *count.entry(i).or_insert(0) += 1;
        }

        let mut total_count = 0;

        for &i in count.values(){

            total_count += i * (i-1) / 2;

        }
        
        total_count
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_identical_pairs() {
        assert_eq!(num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
        assert_eq!(num_identical_pairs(vec![1, 1, 1, 1]), 6);
        assert_eq!(num_identical_pairs(vec![1, 2, 3]), 0);
        assert_eq!(num_identical_pairs(vec![1, 1, 2, 2, 2, 3]), 4);
    }
}

fn main() {
    let nums = [1,2,3,1,1,3];
    let res = num_identical_pairs(nums.to_vec());

    println!("{:?}" , res);
    
}
