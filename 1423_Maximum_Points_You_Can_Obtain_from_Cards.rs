impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let mut score = card_points.iter().take(k as usize).sum();
        let mut r = score;
        let n = card_points.len() as i32;
        for i in 1..=k {
            score -= card_points[(k - i) as usize];
            score += card_points[(n - i) as usize];
            if score > r {
                r = score;
            }
        }
        r
    }
}