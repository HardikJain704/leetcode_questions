pub fn find_min(nums: Vec<i32>) -> i32 {
        binary_search(&nums, 0, nums.len() - 1)
}

pub fn binary_search(nums: &[i32] , start: usize , end : usize) -> i32{
    let mut start = start;
    let mut end = end;

    while start < end {
        let mid = start + (end - start) / 2 as usize;
        if mid == start {
            return  nums[start].min(nums[end])
        }
        if nums[start] == nums[mid] && nums[mid] == nums[end] {
            return binary_search(nums, start, mid).min(binary_search(nums, mid, end))
        }

        if nums[start] <= nums[mid] && nums[mid] <= nums[end] || nums[start] > nums[mid]{
            end = mid;
        } else {
            start = mid;
        }
    }
    nums[start]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min() {
        assert_eq!(find_min(vec![3, 4, 5, 1, 2]), 1);
        assert_eq!(find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
        assert_eq!(find_min(vec![11, 13, 15, 17]), 11);
        assert_eq!(find_min(vec![2, 2, 2, 0, 1]), 0);
        assert_eq!(find_min(vec![1, 2, 2, 2, 2]), 1);
        assert_eq!(find_min(vec![2, 2, 2, 2, 1]), 1);
        assert_eq!(find_min(vec![2, 2, 1, 2, 2]), 1);
    }
}
fn main() {
    
    let nums = vec![2,2,2,0,1];
    let res = find_min(nums);

    println!("{:?}", res);

}
