impl Solution {
    pub fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
        let n = matrix.len();
        for i in 0..n {
            let mut v = vec![false; n + 1];
            for j in 0..n {
                if v[matrix[i][j] as usize] {
                    return false;
                }
                v[matrix[i][j] as usize] = true;
            }
        }
        for j in 0..n {
            let mut v = vec![false; n + 1];
            for i in 0..n {
                if v[matrix[i][j] as usize] {
                    return false;
                }
                v[matrix[i][j] as usize] = true;
            }
        }
        true
    }
}