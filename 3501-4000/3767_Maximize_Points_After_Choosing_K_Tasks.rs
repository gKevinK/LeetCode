impl Solution {
    pub fn max_points(technique1: Vec<i32>, technique2: Vec<i32>, k: i32) -> i64 {
        let n = technique1.len();
        let mut sum = 0;
        let mut diff = vec![];
        for i in 0..n {
            sum += technique2[i] as i64;
            diff.push(std::cmp::Reverse(technique1[i] as i64 - technique2[i] as i64));
        }
        diff.sort_unstable();
        let mut i = 0;
        while i < k as usize || (i < n && diff[i].0 > 0) {
            sum += diff[i].0;
            i += 1;
        }
        sum
    }
}