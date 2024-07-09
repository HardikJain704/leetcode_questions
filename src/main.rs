pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut m = m as usize;
        let mut n = n as usize;
        let mut i = m + n as usize;

        while n > 0 {
            i-= 1;
            if m > 0 && nums1[m-1] > nums2[n-1] {
                m -= 1;
                nums1[i] = nums1[m];
            }
            else{
            n -= 1;
            nums1[i] = nums2[n]
            }
        }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        let m = 3;
        let n = 3;
        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);

        let mut nums1 = vec![1, 2, 4, 5, 6, 0];
        let mut nums2 = vec![3];
        let m = 5;
        let n = 1;
        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1, 2, 3, 4, 5, 6]);

        let mut nums1 = vec![0];
        let mut nums2 = vec![1];
        let m = 0;
        let n = 1;
        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1]);
    }
}

fn main() {

    let mut nums1 = vec![1,2,3,0,0,0];
    let mut nums2 = vec![2,5,6];
    let m = 3;
    let n = 3;

     merge(&mut nums1, m, &mut nums2, n);

    println!("{:?}" , nums1);

}
