impl Solution {
    pub fn max_score(s: String) -> i32 {
        let v: Vec<_> = s.chars().collect();
        let mut score = v.iter().filter(|c| **c == '1').count() as i32;
        let mut r = 0;
        for i in 0..(v.len() - 1) {
            let c = v[i];
            if c == '0' {
                score += 1;
            } else {
                score -= 1;
            }
            if score > r {
                r = score;
            }
        }
        r
    }
}