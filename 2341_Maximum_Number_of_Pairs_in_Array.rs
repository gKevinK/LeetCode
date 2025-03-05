impl Solution {
    pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut m = HashMap::new();
        for n in nums {
            let e = m.entry(n).or_insert(0);
            *e += 1;
        }
        let mut p = 0;
        let mut r = 0;
        for (n, c) in m {
            p += c / 2;
            r += c % 2;
        }
        vec![p, r]
    }
}