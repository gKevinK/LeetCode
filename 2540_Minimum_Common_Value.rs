impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let l1 = nums1.len();
        let l2 = nums2.len();
        let mut i1 = 0;
        let mut i2 = 0;
        while i1 < l1 && i2 < l2 {
            if nums1[i1] == nums2[i2] {
                return nums1[i1];
            } else if nums1[i1] < nums2[i2] {
                i1 += 1;
            } else {
                i2 += 1;
            }
        }
        -1
    }
}