impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        let n = rating.len();
        let mut v1 = vec![0; n];
        let mut v2 = vec![0; n];
        let mut r = 0;
        for i in 0..n {
            for j in 0..i {
                if rating[j] < rating[i] {
                    v1[i] += 1;
                    r += v1[j];
                }
                if rating[j] > rating[i] {
                    v2[i] += 1;
                    r += v2[j];
                }
            }
        }
        r
    }
}