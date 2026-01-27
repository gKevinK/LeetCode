impl Solution {
    pub fn max_active_sections_after_trade(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = s.len();
        let s = s.as_bytes();
        let mut count1 = 0;
        let mut zero = Vec::with_capacity(n / 2 + 1);
        let mut index = vec![0; n];
        let mut i = 0;
        while i < n {
            index[i] = zero.len();
            let mut j = i + 1;
            while j < n && s[j] == s[i] {
                index[j] = zero.len();
                j += 1;
            }
            if s[i] == b'1' {
                count1 += j - i;
            } else {
                zero.push((i, j - i));
            }
            i = j;
        }
        if zero.is_empty() {
            return vec![count1 as i32; queries.len()];
        }
        let base = (zero.len() - 1).next_power_of_two();
        let mut gains = Vec::with_capacity(base * 2);
        gains.resize(base, 0);
        gains.extend((1..zero.len()).map(|i| zero[i - 1].1 + zero[i].1));
        gains.resize(base * 2, 0);
        let mut b = base / 2;
        while b > 0 {
            for i in 0..b {
                gains[b + i] = gains[b * 2 + i * 2].max(gains[b * 2 + i * 2 + 1]);
            }
            b >>= 1;
        }

        let mut res = vec![count1 as i32; queries.len()];
        for i in 0..queries.len() {
            let l = queries[i][0] as usize;
            let r = queries[i][1] as usize;
            if index[l] == index[r] {
                continue;
            }
            let lz = index[l] + if s[l] == b'0' { 1 } else { 0 };
            let rz = index[r];
            if lz + 1 < rz {
                let mut lg = lz;
                let mut rg = rz - 2;
                let mut b = base;
                while b > 0 {
                    if lg > rg {
                        break;
                    }
                    if lg & 1 == 1 {
                        res[i] = res[i].max((count1 + gains[b + lg]) as i32);
                        lg += 1;
                    }
                    if rg & 1 == 0 {
                        res[i] = res[i].max((count1 + gains[b + rg]) as i32);
                        rg -= 1;
                    }
                    lg >>= 1;
                    rg >>= 1;
                    b >>= 1;
                }
            }
            let lc = if s[l] == b'0' { zero[index[l]].0 + zero[index[l]].1 - l } else { 0 };
            let rc = if s[r] == b'0' { r - zero[index[r]].0 + 1 } else { 0 };
            if s[l] == b'0' && s[r] == b'0' && lz == rz {
                res[i] = res[i].max((count1 + lc + rc) as i32);
            }
            if s[l] == b'0' && lz < rz {
                res[i] = res[i].max((count1 + lc + zero[index[l] + 1].1) as i32);
            }
            if s[r] == b'0' && lz + 1 <= rz {
                res[i] = res[i].max((count1 + zero[rz - 1].1 + rc) as i32);
            }
        }
        res
    }
}