pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    let mut path = Vec::new();
    let mut used = vec![false; nums.len()];

    let mut nums_sorted = nums.clone();
    nums_sorted.sort(); // Sort to help skip duplicates

    dfs(&nums_sorted, &mut path, &mut used, &mut res);

    res
}

fn dfs(nums: &Vec<i32>, path: &mut Vec<i32>, used: &mut Vec<bool>, result: &mut Vec<Vec<i32>>) {
    if path.len() == nums.len() {
        result.push(path.clone());
        return;
    }
    
    for i in 0..nums.len() {
        // Skip used elements and duplicates
        if used[i] || (i > 0 && nums[i] == nums[i - 1] && !used[i - 1]) {
            continue;
        }

        path.push(nums[i]);
        used[i] = true;
        dfs(nums, path, used, result);
        path.pop();
        used[i] = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permute_empty() {
        let nums: Vec<i32> = vec![];
        let result = permute(nums);
        let expected: Vec<Vec<i32>> = vec![vec![]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_permute_single() {
        let nums = vec![1];
        let result = permute(nums);
        let expected = vec![vec![1]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_permute_three() {
        let nums = vec![1, 2, 3];
        let mut result = permute(nums);
        let mut expected = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];
        result.sort();
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_permute_duplicates() {
        let nums = vec![1, 1, 2];
        let mut result = permute(nums);
        let mut expected = vec![
            vec![1, 1, 2],
            vec![1, 2, 1],
            vec![2, 1, 1],
        ];
        result.sort();
        expected.sort();
        assert_eq!(result, expected);
    }
}

fn main() {
    let nums = vec![1, 2, 3];
    let res: Vec<Vec<i32>> = permute(nums);

    println!("{:?}", res);
}
