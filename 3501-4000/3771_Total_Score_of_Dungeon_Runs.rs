impl Solution {
    pub fn total_score(hp: i32, damage: Vec<i32>, requirement: Vec<i32>) -> i64 {
        let n = damage.len();
        let hp = hp as i64;
        let mut psum = vec![0; n + 1];
        let mut sum = 0;
        let mut res = 0;
        for i in 0..n {
            sum += damage[i] as i64;
            let req = requirement[i] as i64;
            if hp >= req {
                res += i + 1 - psum[..=i].partition_point(|p| *p < req + sum - hp);
            }
            psum[i + 1] = sum;
        }
        res as _
    }
}