impl Solution {
    pub fn lex_smallest(s: String) -> String {
        let bytes = s.bytes().collect::<Vec<_>>();
        let n = bytes.len();
        let mut res = bytes.clone();

        for k in 1..=n {
            let mut cand = bytes.clone();
            cand[0..k].reverse();
            if cand < res {
                res = cand;
            }
            cand = bytes.clone();
            cand[(n - k)..n].reverse();
            if cand < res {
                res = cand;
            }
        }
        String::from_utf8(res).unwrap()
    }
}