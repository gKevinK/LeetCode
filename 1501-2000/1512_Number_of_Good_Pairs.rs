impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut m = std::collections::HashMap::new();
        let mut r = 0;
        for n in nums {
            let e = m.entry(n).or_insert(0);
            r += *e;
            *e += 1;
        }
        r
    }
}