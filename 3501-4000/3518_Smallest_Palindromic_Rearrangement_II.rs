impl Solution {
    pub fn smallest_palindrome(s: String, k: i32) -> String {
        let mut count = vec![0; 26];
        for b in s.bytes() {
            count[(b - b'a') as usize] += 1;
        }
        let mut mid = Option::None;
        let mut n = 0;
        for i in 0..26 {
            if count[i] % 2 == 1 {
                mid = Some((b'a' + i as u8) as char);
                count[i] -= 1;
            }
            count[i] /= 2;
            n += count[i];
        }

        let mut k = k as i64;
        let mut v = vec![0u8; n as usize];
        for i in 0..v.len() {
            for c in 0..26 {
                if count[c] > 0 {
                    count[c] -= 1;
                    let p = Self::calc_p(&count, n - i as i32 - 1, k);
                    if p >= k {
                        v[i] = b'a' + c as u8;
                        break;
                    }
                    k -= p;
                    count[c] += 1;
                }
            }
            if i == 0 && v[i] == 0 {
                return "".to_string();
            }
        }
        let mut res = String::from_utf8_lossy(&v).into_owned();
        if let Some(c) = mid {
            res.push(c);
        }
        v.reverse();
        res.push_str(&String::from_utf8(v).unwrap());
        res
    }

    fn calc_p(count: &Vec<i32>, mut n: i32, limit: i64) -> i64 {
        let mut p = 1;
        for i in 0..26 {
            let m = count[i].min(n - count[i]);
            for x in 0..m {
                p *= (n - x) as i64;
                p /= (x + 1) as i64;
                if p > limit {
                    return p;
                }
            }
            n -= count[i];
        }
        p
    }
}