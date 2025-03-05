impl Solution {
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let mut m = std::collections::HashMap::new();
        for n in nums1 {
            *m.entry(n).or_insert(0) |= 1;
        }
        for n in nums2 {
            *m.entry(n).or_insert(0) |= 2;
        }
        for n in nums3 {
            *m.entry(n).or_insert(0) |= 4;
        }
        let mut r = Vec::new();
        for (k, v) in m {
            if (v & 1) + ((v & 2) >> 1) + ((v & 4) >> 2) >= 2 {
                r.push(k);
            }
        }
        r
    }
}