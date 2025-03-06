impl Solution {
    pub fn max_sum(nums: Vec<i32>, k: i32, m: i32) -> i32 {
        let n = nums.len();
        let uk = k as usize;
        let um = m as usize;
        let mut ns = vec![0; n + 1];
        for i in 0..n {
            ns[i + 1] = ns[i] + nums[i];
        }
        let mut dp = vec![vec![std::i32::MIN; n + 1]; uk + 1];
        dp[0] = vec![0; n + 1];
        let mut sum = 0;
        for ik in 1..=uk {
            dp[ik][ik * um] = ns[ik * um];

            let mut smax_i = (ik - 1) * um;
            let mut smax_v = dp[ik - 1][smax_i] - ns[smax_i];
            for i in (ik * um + 1)..=n {
                dp[ik][i] = dp[ik][i - 1];
                let check_i = i - um;
                let check_v = dp[ik - 1][check_i] - ns[check_i];
                if check_v > smax_v {
                    smax_i = check_i;
                    smax_v = check_v;
                }
                let cand = ns[i] + smax_v;
                dp[ik][i] = std::cmp::max(dp[ik][i], cand);
            }
        }
        // println!("{:?}", dp);
        dp[k as usize][nums.len()]
    }
}