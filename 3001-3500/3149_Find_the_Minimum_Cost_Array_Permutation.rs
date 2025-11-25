impl Solution {
    pub fn find_permutation(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let all = (1 << n - 1) - 1;
        let MAX = 10000;
        let mut dp = vec![vec![(MAX, 0); 1 << n - 1]; n];
        for i in 0..n {
            dp[i][all] = ((i as i32 - nums[0]).abs(), 0);
        }
        for x in (1..(n - 1)).rev() {
            for m in 1..=all {
                if m.count_ones() as usize != x {
                    continue;
                }
                for curr in 1..n {
                    if m & (1 << curr - 1) == 0 {
                        continue;
                    }
                    for next in 1..n {
                        if m & (1 << next - 1) == 0 {
                            let val = dp[next][m | (1 << next - 1)].0 + (curr as i32 - nums[next]).abs();
                            if val < dp[curr][m].0 {
                                dp[curr][m] = (val, next as i32);
                            }
                        }
                    }
                }
            }
        }
        for next in 1..n {
            let val = dp[next][1 << next - 1].0 + nums[next].abs();
            if val < dp[0][0].0 {
                dp[0][0] = (val, next as i32);
            }
        }
        let mut res = vec![0; n];
        let mut mask = 0;
        for i in 1..n {
            res[i] = dp[res[i - 1] as usize][mask].1;
            mask += 1 << res[i] as usize - 1;
        }
        res
    }
}