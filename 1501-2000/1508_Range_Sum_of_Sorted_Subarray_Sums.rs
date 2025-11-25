impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let n = n as usize;
        let mut s = vec![0; n + 1];
        for i in 0..n {
            s[i + 1] = s[i] + nums[i];
        }
        let mut v = vec![];
        for i in 1..=n as usize {
            for j in 0..i {
                v.push(s[i] - s[j]);
            }
        }
        let (l, m, _) = v.select_nth_unstable(right as usize - 1);
        if left == right {
            return *m;
        }
        l.select_nth_unstable(left as usize - 1);

        const MOD: i32 = 1_000_000_007;
        v[(left - 1) as usize..right as usize].iter().fold(0, |acc, &x| (acc + x) % MOD) as _
    }
}