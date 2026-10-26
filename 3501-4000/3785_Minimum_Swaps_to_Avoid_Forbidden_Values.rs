impl Solution {
    pub fn min_swaps(nums: Vec<i32>, forbidden: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let n = nums.len();
        let mut count_nums = HashMap::new();
        let mut count_forbid = HashMap::new();
        for &v in &nums {
            *count_nums.entry(v).or_insert(0) += 1;
        }
        for &v in &forbidden {
            *count_forbid.entry(v).or_insert(0) += 1;
        }
        for (&v, &c) in &count_nums {
            let c2 = count_forbid.get(&v).cloned().unwrap_or(0);
            if c + c2 > n {
                return -1;
            }
        }

        let mut eq = HashMap::new();
        let mut k = 0;
        let mut m = 0;
        for i in 0..n {
            if nums[i] == forbidden[i] {
                let c = eq.entry(nums[i]).or_insert(0);
                *c += 1;
                k += 1;
                m = m.max(*c);
            }
        }
        ((k + 1) / 2).max(m) as i32
    }
}