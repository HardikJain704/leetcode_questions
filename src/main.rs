// This is optimised solution where i am traversing only once which brings down the TC - O(N) using hashmap.
use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    for(i, &num) in nums.iter().enumerate() {
        let remaining = target - num;

        if let Some(&index) = map.get(&remaining){
            return vec![i as i32 , index];``
        }
        map.insert(num,i as i32);

    
    }
    return vec![];

}

fn main(){
    let nums = vec![2,7,11,15];
    let target = 9;
    let result = two_sum(nums, target);
    println!("{:?}", result);
}