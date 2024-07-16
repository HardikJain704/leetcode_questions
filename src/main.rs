pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut answer: Vec<i32> = Vec::new();

        for (i, n1) in nums1.iter().enumerate() {
            let mut pos = 0;
            if let Some(p) = nums2.iter().position(|&x| x == *n1){
                pos = p;
            }

            for p in pos..nums2.len(){
                if nums2[p] > *n1 {
                    answer.push(nums2[p]);
                    break;
                }
            }
            
            if answer.len() != i + 1 {
                answer.push(-1);
            }

        }
        answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let nums1 = vec![4, 1, 2];
        let nums2 = vec![1, 3, 4, 2];
        let result = next_greater_element(nums1, nums2);
        assert_eq!(result, vec![-1, 3, -1]);
    }

    #[test]
    fn test_example2() {
        let nums1 = vec![2, 4];
        let nums2 = vec![1, 2, 3, 4];
        let result = next_greater_element(nums1, nums2);
        assert_eq!(result, vec![3, -1]);
    }

    #[test]
    fn test_all_smaller_elements() {
        let nums1 = vec![4, 5, 6];
        let nums2 = vec![1, 2, 3];
        let result = next_greater_element(nums1, nums2);
        assert_eq!(result, vec![-1, -1, -1]);
    }

    #[test]
    fn test_all_greater_elements() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![1,2,3,4, 5, 6];
        let result = next_greater_element(nums1, nums2);
        assert_eq!(result, vec![2, 3, 4]);
    }

    #[test]
    fn test_mixed_elements() {
        let nums1 = vec![2, 4, 1];
        let nums2 = vec![3, 2, 1, 5, 4];
        let result = next_greater_element(nums1, nums2);
        assert_eq!(result, vec![5,-1, 5]);
    }

    #[test]
    fn test_single_element() {
        let nums1 = vec![1];
        let nums2 = vec![1, 2, 3, 4];
        let result = next_greater_element(nums1, nums2);
        assert_eq!(result, vec![2]);
    }
}

fn main() {
    let nums1 = vec![4,1,2];
    let nums2 = vec![1,3,4,2];

    let res = next_greater_element(nums1, nums2);

    println!("{:?}" , res);

}
