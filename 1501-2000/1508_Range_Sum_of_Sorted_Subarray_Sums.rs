impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let mut s = vec![0];
        for i in nums {
            s.push(s.last().unwrap() + i);
        }
        let mut v = vec![];
        for i in 1..=n as usize {
            for j in 0..i {
                v.push(s[i] - s[j]);
            }
        }
        v.sort();
        let mut r = 0;
        let MOD = 1_000_000_007;
        for i in (left - 1)..right {
            r += v[i as usize];
            r %= MOD;
        }
        r
    }
}