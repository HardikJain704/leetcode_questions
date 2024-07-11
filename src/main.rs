pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        
        if nums.len() == 0 || nums.len() >= 10{
            return vec![vec![]];

        }

        let mut res: Vec<Vec<i32>> = vec![vec![]];

        for i in nums {
            let mut all_possible_subsets = res.clone().into_iter().map(|mut x| {

               x.push(i);
               x

            }).collect();
            res.append(&mut all_possible_subsets);
        }
        res

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let nums: Vec<i32> = vec![];
        let result = subsets(nums);
        let expected: Vec<Vec<i32>> = vec![vec![]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_single_element() {
        let nums = vec![1];
        let result = subsets(nums);
        let expected: Vec<Vec<i32>> = vec![vec![], vec![1]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_two_elements() {
        let nums = vec![1, 2];
        let result = subsets(nums);
        let expected: Vec<Vec<i32>> = vec![vec![], vec![1], vec![2], vec![1, 2]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_three_elements() {
        let nums = vec![1, 2, 3];
        let result = subsets(nums);
        let expected: Vec<Vec<i32>> = vec![
            vec![], vec![1], vec![2], vec![1, 2], 
            vec![3], vec![1, 3], vec![2, 3], vec![1, 2, 3]
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_large_input() {
        let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let result = subsets(nums);
        assert_eq!(result.len(), 512); // 2^9 subsets
    }

    #[test]
    fn test_exceeding_max_length() {
        let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result = subsets(nums);
        let expected: Vec<Vec<i32>> = vec![vec![]];
        assert_eq!(result, expected);
    }
}


fn main() {
 
    let nums = vec![1,2,3,4];
    let res = subsets(nums);

    println!("{:?}" , res);

}
