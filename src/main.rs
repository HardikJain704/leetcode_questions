pub fn majority_element(nums: Vec<i32>) -> i32 {

       let mut count = 0;
       let mut ele = 0;

       for &i in nums.iter(){
        if count == 0 {
            ele = i;
            count = 1;
        }
        else if ele == i {
            count += 1;
        }
        else{
            count -= 1;
        }
       } 
       ele.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_majority_element() {
        assert_eq!(majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
        assert_eq!(majority_element(vec![3, 3, 4, 2, 4, 4, 2, 4, 4]), 4);
        assert_eq!(majority_element(vec![3, 3, 3, 3, 2, 2, 2, 2, 2]), 2);
    }
}

fn main() {
    let nums = [2,2,1,1,1,2,2];
    let res = majority_element(nums.to_vec());
    
    println!("{:?}" , res);

}
