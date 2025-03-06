impl Solution {
    pub fn is_possible_to_rearrange(s: String, t: String, k: i32) -> bool {
        let k = k as usize;
        let l = s.len() / k;
        let mut c = std::collections::HashMap::new();
        for i in 0..k {
            let sub = &s[(i * l)..((i + 1) * l)];
            *c.entry(sub).or_insert(0) += 1;
        }
        for i in 0..k {
            let sub = &t[(i * l)..((i + 1) * l)];
            let e = c.entry(sub).or_insert(0);
            if *e == 0 {
                return false;
            }
            *e -= 1;
        }
        true
    }
}