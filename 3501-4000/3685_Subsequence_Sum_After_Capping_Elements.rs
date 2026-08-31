impl Solution {
    pub fn subsequence_sum_after_capping(mut nums: Vec<i32>, k: i32) -> Vec<bool> {
        nums.sort();
        let n = nums.len();
        let ku = k as usize;
        let mut dp = vec![false; ku + 1];
        dp[0] = true;
        let mut d1 = vec![0];
        let mut res = vec![false; n];
        let mut p = 0;
        for i in 1..=n as i32 {
            while p < n && nums[p] < i {
                let num = nums[p];
                let len = d1.len();
                for j in 0..len {
                    let next = d1[j] + num;
                    if next <= k && !dp[next as usize] {
                        dp[next as usize] = true;
                        d1.push(next);
                    }
                }
                p += 1;
            }
            if dp[ku] {
                res[(i as usize - 1)..].fill(true);
                break;
            }
            for x in (0..=(k / i).min((n - p) as i32)).rev() {
                if dp[(k - x * i) as usize] {
                    res[i as usize - 1] = true;
                    break;
                }
            }
        }
        res
    }
}