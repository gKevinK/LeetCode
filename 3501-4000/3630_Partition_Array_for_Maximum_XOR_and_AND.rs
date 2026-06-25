impl Solution {
    pub fn maximize_xor_and_xor(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let all = (1 << n as i32) - 1;
        let mut base = vec![];
        let mut res = 0;
        for mask in 0..=all {
            let mut and = -1;
            let mut xor = 0;
            for i in 0..n {
                if mask & (1 << i) > 0 {
                    and &= nums[i];
                } else {
                    xor ^= nums[i];
                }
            }
            if mask == 0 {
                and = 0;
            }
            base.clear();
            let xor_inv = !xor;
            for i in 0..n {
                if mask & (1 << i) == 0 {
                    let mut new = nums[i] & xor_inv;
                    for &b in &base {
                        new = new.min(new ^ b);
                    }
                    if new > 0 {
                        base.push(new);
                        let mut x = base.len();
                        while x > 0 && base[x - 1] < new {
                            x -= 1;
                        }
                        base.insert(x, new);
                    }
                }
            }
            let mut max = 0;
            for &b in &base {
                max = max.max(max ^ b);
            }
            res = res.max(and as i64 + xor as i64 + max as i64 * 2);
        }
        res
    }
}