impl Solution {
    pub fn max_num_of_substrings(s: String) -> Vec<String> {
        let s = s.as_bytes();
        let n = s.len();
        let MAX = 1_000_000;
        let mut l = [MAX; 26];
        let mut r = [0; 26];
        for (i, byte) in s.iter().enumerate() {
            let u = (*byte - b'a') as usize;
            l[u] = l[u].min(i);
            r[u] = r[u].max(i);
        }
        let mut cands = vec![];
        for d in 0..26 {
            if l[d] > r[d] {
                continue;
            }
            let left = l[d];
            let mut right = r[d];
            let mut valid = true;
            let mut i = left;
            while i <= right {
                let u = (s[i] - b'a') as usize;
                if l[u] < left {
                    valid = false;
                    break;
                }
                right = right.max(r[u]);
                i += 1;
            }
            if valid {
                cands.push((left, right));
            }
        }

        cands.sort_by_key(|&(_, right)| right);

        let mut res = vec![];
        let mut last = 0;
        for (left, right) in cands {
            if last <= left {
                res.push(String::from_utf8_lossy(&s[left..=right]).into_owned());
                last = right + 1;
            }
        }
        res
    }
}