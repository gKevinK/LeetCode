impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        let k = k as usize;
        let mut res = vec![0; k + 2];
        let mut dp = HashMap::new();
        for n in nums {
            for i in (0..=k).rev() {
                let e = dp.entry(n).or_insert(vec![0; k + 1]);
                e[i] = std::cmp::max(e[i], res[i]) + 1;
                res[i + 1] = std::cmp::max(res[i + 1], e[i]);
            }
        }
        *res.last().unwrap()
    }
}