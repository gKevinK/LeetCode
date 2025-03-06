impl Solution {
    pub fn added_integer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let m1 = nums1.iter().min().unwrap();
        let m2 = nums2.iter().min().unwrap();
        *m2 - *m1
    }
}