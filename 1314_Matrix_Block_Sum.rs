impl Solution {
    pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let mut s = vec![vec![0; n + 1]; m + 1];
        for i in 0..m {
            for j in 0..n {
                s[i + 1][j + 1] = s[i][j + 1] + s[i + 1][j] - s[i][j] + mat[i][j];
            }
        }
        let uk = k as usize;
        (0..m).map(|i| (0..n).map(|j| {
            let l = if j > uk { j - uk } else { 0 };
            let r = if j + uk < n { j + uk + 1 } else { n };
            let u = if i > uk { i - uk } else { 0 };
            let d = if i + uk < m { i + uk + 1 } else { m };
            s[d][r] - s[d][l] - s[u][r] + s[u][l]
        }).collect()).collect()
    }
}