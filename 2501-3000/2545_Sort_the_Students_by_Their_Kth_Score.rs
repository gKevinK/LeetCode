impl Solution {
    pub fn sort_the_students(mut score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let uk = k as usize;
        score.sort_by(|a, b| b[uk].partial_cmp(&a[uk]).unwrap());
        score
    }
}