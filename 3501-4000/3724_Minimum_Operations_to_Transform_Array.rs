impl Solution {
    pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let n = nums1.len();
        let mut res = 0;
        let end = nums2[n];
        let mut need = 100_002;
        for i in 0..n {
            let a = nums1[i].min(nums2[i]);
            let b = nums1[i].max(nums2[i]);
            res += (b - a) as i64;
            need = need.min(if a <= end && end <= b { 0 } else { (end - b).abs().min((a - end).abs()) });
        }
        res + need as i64 + 1
    }
}