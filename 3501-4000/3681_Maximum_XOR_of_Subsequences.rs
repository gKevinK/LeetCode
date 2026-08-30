impl Solution {
    pub fn max_xor_subsequences(nums: Vec<i32>) -> i32 {
        let mut base = vec![];
        for &num in &nums {
            let mut x = num;
            for &b in &base {
                x = x.min(x ^ b);
            }
            if x > 0 {
                base.push(x);
            }
        }
        let mut res = 0;
        for &b in &base {
            res = res.max(res ^ b);
        }
        res
    }
}