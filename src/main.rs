use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut hash_set = HashSet::new();
        !nums.iter().all(|i| hash_set.insert(i))
}

fn main() {
    
    let nums = vec![1,2,3,4];
    let res = contains_duplicate(nums);

    println!("{:?}" , res);

    
}
