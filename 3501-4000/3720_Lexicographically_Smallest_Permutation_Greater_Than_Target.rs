impl Solution {
    pub fn lex_greater_permutation(s: String, target: String) -> String {
        let mut s_bytes = s.into_bytes();
        let t_bytes = target.into_bytes();
        let n = s_bytes.len();
        let mut count = [0; 26];
        for &c in &s_bytes {
            count[(c - b'a') as usize] += 1;
        }
        let mut res = Vec::new();
        if Self::dfs(&mut count, &t_bytes, 0, &mut res) {
            String::from_utf8(res).unwrap()
        } else {
            String::new()
        }
    }

    fn dfs(mut count: &mut [i32; 26], target: &[u8], pos: usize, mut current: &mut Vec<u8>) -> bool {
        if pos == target.len() {
            return &current[..] > target;
        }
        let tbyte = target[pos];

        let ib = (tbyte - b'a') as usize;
        if count[ib] > 0 {
            count[ib] -= 1;
            current.push(tbyte);
            if Self::dfs(&mut count, &target, pos + 1, &mut current) {
                return true;
            }
            current.pop();
            count[ib] += 1;
        }

        for b in (tbyte + 1)..=b'z' {
            let ib = (b - b'a') as usize;
            if count[ib] > 0 {
                count[ib] -= 1;
                current.push(b);
                for i in b'a'..=b'z' {
                    for _ in 0..count[(i - b'a') as usize]  {
                        current.push(i);
                    }
                }
                return true;
            }
        }
        false
    }
}