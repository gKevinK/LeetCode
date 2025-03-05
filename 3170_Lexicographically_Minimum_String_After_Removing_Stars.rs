impl Solution {
    pub fn clear_stars(s: String) -> String {
        let mut record = vec![vec![]; 26];
        let mut v = s.into_bytes();
        for i in 0..v.len() {
            if v[i] == b'*' {
                for j in 0..26 {
                    if let Some(k) = record[j].pop() {
                        v[k] = b'*';
                        break;
                    }
                }
            } else {
                record[(v[i] - b'a') as usize].push(i);
            }
        }
        let res: Vec<u8> = v.into_iter().filter(|&b| b != b'*').collect();
        String::from_utf8(res).unwrap()
    }
}