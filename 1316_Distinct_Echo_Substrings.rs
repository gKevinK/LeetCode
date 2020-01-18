impl Solution {
    pub fn distinct_echo_substrings(text: String) -> i32 {
        let v: Vec<_> = text.chars().collect();
        let mut s = std::collections::HashSet::new();
        let mut r = 0;
        for i in 1..=(v.len() / 2) {
            let mut l = 0;
            for j in 0..(v.len() - i) {
                if v[j] == v[j + i] {
                    l += 1;
                    if l >= i && l < i * 2 {
                        s.insert(&text[(j + 1)..(j + i + 1)]);
                    }
                } else {
                    l = 0;
                }
            }
        }
        s.len() as i32
    }
}