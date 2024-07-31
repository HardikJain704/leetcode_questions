pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
         return 0;         
}

    let min_num = nums.iter().min().unwrap();
    let max_num = nums.iter().max().unwrap();

    let bucket_size = ((max_num - min_num) / (nums.len() as i32 - 1)).max(1);
    let bucket_count = ((max_num - min_num) / bucket_size + 1) as usize;

    let mut buckets_min = vec![i32::MAX; bucket_count];
    let mut buckets_max = vec![i32::MIN;  bucket_count];

    for &num in &nums {
        let idx = ((num - min_num) / bucket_size) as usize;

        buckets_min[idx] = buckets_min[idx].min(num);
        buckets_max[idx] = buckets_max[idx].max(num);


    }

    let mut max_gap = 0;
    let mut prev_max = min_num;

    for i in 0..bucket_count {
        if buckets_min[i] == i32::MAX {
            continue;
        }

        max_gap = max_gap.max(buckets_min[i] - prev_max);
        prev_max = &buckets_max[i];
    }

    max_gap
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_gap1() {
        let nums = vec![3, 6, 9, 1];
        assert_eq!(maximum_gap(nums), 3);
    }

    #[test]
    fn test_maximum_gap2() {
        let nums = vec![10];
        assert_eq!(maximum_gap(nums), 0);
    }

    #[test]
    fn test_maximum_gap3() {
        let nums = vec![1, 1, 1, 1];
        assert_eq!(maximum_gap(nums), 0);
    }

    #[test]
    fn test_maximum_gap4() {
        let nums = vec![1, 10000000];
        assert_eq!(maximum_gap(nums), 9999999);
    }

    #[test]
    fn test_maximum_gap5() {
        let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(maximum_gap(nums), 1);
    }

    #[test]
    fn test_maximum_gap6() {
        let nums = vec![-3, -6, -9, -1];
        assert_eq!(maximum_gap(nums), 3);
    }

    #[test]
    fn test_maximum_gap_empty() {
        let nums: Vec<i32> = vec![];
        assert_eq!(maximum_gap(nums), 0);
    }

    #[test]
    fn test_maximum_gap_single_element() {
        let nums = vec![42];
        assert_eq!(maximum_gap(nums), 0);
    }
}

fn main() {
    let nums = vec![3,6,9,1];
    let res = maximum_gap(nums);

    println!("{:?}" , res);

}
