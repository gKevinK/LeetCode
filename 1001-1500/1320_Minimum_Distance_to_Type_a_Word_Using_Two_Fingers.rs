impl Solution {
    fn distance(a: i32, b: i32) -> i32 {
        let xa = a / 6;
        let ya = a % 6;
        let xb = b / 6;
        let yb = b % 6;
        (xa - xb).abs() + (ya - yb).abs()
    }
    pub fn minimum_distance(word: String) -> i32 {
        use std::cmp::min;

        let mut dp = vec![0; 26];
        let mut last = -1;
        for _c in word.chars() {
            let c = _c as i32 - 'A' as i32;
            let mut dp2 = vec![std::i32::MAX; 26];
            if last != -1 {
                for i in 0..26 {
                    dp2[i as usize] = min(dp[i as usize] + Self::distance(last, c), dp2[i as usize]);
                    dp2[last as usize] = min(dp[i as usize] + Self::distance(i, c), dp2[last as usize]);
                }
                dp = dp2;
            }
            last = c;
        }
        let mut r = std::i32::MAX;
        for i in dp {
            if i < r {
                r = i;
            }
        }
        r
    }
}