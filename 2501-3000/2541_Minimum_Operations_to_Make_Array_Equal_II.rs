impl Solution {
    pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let mut inc = 0;
        let mut dec = 0;
        let n = nums1.len();
        for i in 0..n {
            let diff = nums2[i] - nums1[i];
            if diff != 0 {
                if k == 0 || diff % k != 0 {
                    return -1;
                }
                if diff > 0 {
                    inc += (diff / k) as i64;
                } else {
                    dec -= (diff / k) as i64;
                }
            }
        }
        if inc == dec { inc } else { -1 }
    }
}