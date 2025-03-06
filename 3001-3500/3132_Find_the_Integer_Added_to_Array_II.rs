impl Solution {
    fn check(nums1: &Vec<i32>, nums2: &Vec<i32>, k: i32) -> bool {
        let mut drop = 0;
        let mut j = 0;
        for i in 0..nums1.len() {
            if j >= nums2.len() {
                drop += 1;
                continue;
            }
            if nums1[i] + k == nums2[j] {
                j += 1;
            } else {
                drop += 1;
            }
        }
        drop == 2
    }
    pub fn minimum_added_integer(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> i32 {
        nums1.sort_unstable();
        nums2.sort_unstable();
        let mut res = 10000;
        for i in 0..3 {
            let x = nums2[0] - nums1[i];
            if Solution::check(&nums1, &nums2, x) {
                res = std::cmp::min(res, x);
            }
        }
        res
    }
}