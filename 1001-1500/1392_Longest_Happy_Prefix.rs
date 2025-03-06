impl Solution {
    pub fn longest_prefix(s: String) -> String {
        let c: Vec<_> = s.chars().collect();
        let mut v = vec![0; c.len()];
        for i in 1..c.len() {
            let mut t = i as i32 - 1;
            while t > 0 {
                if c[i] == c[v[t as usize] as usize] {
                    v[i] = v[t as usize] + 1;
                    break;
                }
                t = v[t as usize] - 1;
            }
            if t == 0 && c[0] == c[i] {
                v[i] = 1;
            }
        }
        c.iter().take(*v.last().unwrap() as usize).collect()
    }
}