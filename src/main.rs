pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        
    let n = nums1.len();
    let m = nums2.len();

    if n > m {

    return find_median_sorted_arrays(nums2, nums1);

    }

    let mut low = 0;
    let mut high = n;
    let total = n + m;
    let half = (total + 1) / 2;
    
    while low <= high {
        let mid1 = (low + high ) / 2;
        let mid2 = half as isize - mid1 as isize;

        let l1 = if mid1 > 0 {
            nums1[mid1 - 1] 
        } else {
            i32::MIN

        };
        
        let l2 =  if mid2 > 0 {
            nums2[mid2 as usize - 1]
        } else {
         i32::MIN

        };

        let r1 = if mid1 < n {
        nums1[mid1]
        } else {

            i32::MAX

        };


        let r2 = if mid2 < m as isize {

            nums2[mid2 as usize]
        } else {

            i32::MAX
        };

        if l1 <= r2 && l2 <= r1 {

            if total % 2 == 0 {
                return (l1.max(l2) as f64 + r1.min(r2) as f64) / 2.0;
            } else {

                return l1.max(l2) as f64; 
            }

        } else if l1 > r2 {
            high = mid1 - 1;
         } else {
            low = mid1 + 1; 
         }

    } 
    unreachable!()
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]

    fn test_find_median_sorted_array(){

        assert_eq!(find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
        assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
        assert_eq!(find_median_sorted_arrays(vec![0, 0], vec![0, 0]), 0.0);
        assert_eq!(find_median_sorted_arrays(vec![], vec![1]), 1.0);
        assert_eq!(find_median_sorted_arrays(vec![2], vec![]), 2.0);
        assert_eq!(find_median_sorted_arrays(vec![1, 3], vec![2, 7]), 2.5);
        assert_eq!(find_median_sorted_arrays(vec![1, 2, 6], vec![3, 4, 5]), 3.5);

    }
}
fn main() {

    let nums1 = vec![1,3];
    let nums2 = vec![2];

    let res = find_median_sorted_arrays(nums1, nums2);

    println!("{:?}" , res);


}




