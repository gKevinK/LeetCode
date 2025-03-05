impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut i1 = 0;
        let mut i2 = 0;
        let mut res = vec![];
        let l1 = nums1.len();
        let l2 = nums2.len();
        while i1 < l1 || i2 < l2 {
            if i1 >= l1 {
                res.push(nums2[i2].clone());
                i2 += 1;
            } else if i2 >= l2 {
                res.push(nums1[i1].clone());
                i1 += 1;
            } else if nums1[i1][0] == nums2[i2][0] {
                res.push(vec![nums1[i1][0], nums1[i1][1] + nums2[i2][1]]);
                i1 += 1;
                i2 += 1;
            } else if nums1[i1][0] < nums2[i2][0] {
                res.push(nums1[i1].clone());
                i1 += 1;
            } else {
                res.push(nums2[i2].clone());
                i2 += 1;
            }
        }
        res
    }
}