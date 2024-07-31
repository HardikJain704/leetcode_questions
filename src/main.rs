pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut low = 0;
        let mut high = nums.len() - 1;
        let mut i = 0;

        while i <= high {
            if nums[i] == 0 {
                nums.swap(i , low);
                low += 1;
                i += 1;
            } else if nums[i] == 2 {
                nums.swap(i , high);
                if high == 0 {
                    break;
                }
                high -= 1;

            } else {
                i += 1;

            }
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_colors1() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn test_sort_colors2() {
        let mut nums = vec![2, 0, 1];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 1, 2]);
    }

    #[test]
    fn test_sort_colors3() {
        let mut nums = vec![0];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0]);
    }

    #[test]
    fn test_sort_colors4() {
        let mut nums = vec![1];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![1]);
    }

    #[test]
    fn test_sort_colors5() {
        let mut nums = vec![2, 2, 2, 2];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![2, 2, 2, 2]);
    }

    #[test]
    fn test_sort_colors6() {
        let mut nums = vec![0, 0, 0, 0];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 0, 0]);
    }

    #[test]
    fn test_sort_colors7() {
        let mut nums = vec![1, 1, 1, 1];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![1, 1, 1, 1]);
    }
}

fn main() {
    
    let mut nums = vec![2,0,2,1,1,0];

   sort_colors(&mut nums);

    println!("{:?}" , nums);

}
