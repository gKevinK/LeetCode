impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let v: Vec<usize> = s.bytes().map(|b| (b - b'a') as usize).collect();
        let mut i = 0;
        let mut n = vec![ -1, -1, -1 ];
        let mut r = 0;
        while i < v.len() {
            n[v[i]] = i as i32;
            i += 1;
            let x = n[0].min(n[1].min(n[2]));
            if x >= 0 {
                r += x + 1;
            }
        }
        r
    }
}