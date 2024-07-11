pub fn max_sub_array(nums: Vec<i32>) -> i32 {

    if nums.len() <= 0 {
        return 0;
    }

    let mut max_sum = nums[0]; 
    let mut curr_sum = 0;

    for i in 0..nums.len(){
        curr_sum = std::cmp::max(curr_sum , 0) + nums[i];
        max_sum = std::cmp::max(max_sum , curr_sum.try_into().unwrap());

    }
    max_sum


        
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sub_array() {
        assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(max_sub_array(vec![1]), 1);
        assert_eq!(max_sub_array(vec![5, 4, -1, 7, 8]), 23);
        assert_eq!(max_sub_array(vec![]), 0);
        assert_eq!(max_sub_array(vec![-1, -2, -3, -4]), -1);
        assert_eq!(max_sub_array(vec![8, -19, 5, -4, 20]), 21);
    }
}


fn main() {
    let nums = vec![];

    let res = max_sub_array(nums);

    println!("{:?}" , res);

}
