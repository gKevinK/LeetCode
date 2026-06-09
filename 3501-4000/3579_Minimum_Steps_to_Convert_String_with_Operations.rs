impl Solution {
    pub fn min_operations(word1: String, word2: String) -> i32 {
        let w1 = word1.as_bytes();
        let w2 = word2.as_bytes();
        let n = w1.len();
        let mut dp = vec![vec![0; n]; n];
        for i in 0..n {
            let mut m = [[0; 26]; 26];
            let mut count = 0;
            for j in i..n {
                let c1 = (w1[j] - b'a') as usize;
                let c2 = (w2[j] - b'a') as usize;
                if c1 != c2 {
                    count += 1;
                    if m[c2][c1] > 0 {
                        m[c2][c1] -= 1;
                        count -= 1;
                    } else {
                        m[c1][c2] += 1;
                    }
                }
                dp[i][j] = count;
            }
        }
        for x in 0..(n * 2 - 1) {
            let mut i = (x + 1) / 2;
            let mut j = x / 2;
            let mut m = [[0; 26]; 26];
            let mut count = 1;
            if i == j {
                let c1 = (w1[i] - b'a') as usize;
                let c2 = (w2[i] - b'a') as usize;
                if c1 != c2 {
                    count += 1;
                    m[c1][c2] += 1;
                }
            }
            while i > 0 && j + 1 < n {
                i -= 1;
                j += 1;
                for (c1, c2) in [(w1[i], w2[j]), (w1[j], w2[i])] {
                    let c1 = (c1 - b'a') as usize;
                    let c2 = (c2 - b'a') as usize;
                    if c1 != c2 {
                        count += 1;
                        if m[c2][c1] > 0 {
                            m[c2][c1] -= 1;
                            count -= 1;
                        } else {
                            m[c1][c2] += 1;
                        }
                    }
                }
                dp[i][j] = dp[i][j].min(count);
            }
        }

        let mut d2 = vec![100; n + 1];
        d2[0] = 0;
        for i in 1..=n {
            for j in 0..i {
                d2[i] = d2[i].min(d2[j] + dp[j][i - 1]);
            }
        }
        d2[n]
    }
}