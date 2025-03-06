impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut sum = vec![vec![0; n + 1]; m + 1];
        for i in 0..m {
            for j in 0..n {
                sum[i + 1][j + 1] = sum[i][j + 1] + sum[i + 1][j] - sum[i][j] + matrix[i][j];
            }
        }
        let mut r = 0;
        for i1 in 0..m {
            for i2 in (i1 + 1)..=m {
                let mut map = std::collections::HashMap::new();
                map.insert(0, 1);
                for j in 1..=n {
                    let s = sum[i2][j] - sum[i1][j];
                    r += match map.get(&(s - target)) {
                        Some(v) => *v,
                        None => 0
                    };
                    *map.entry(s).or_insert(0) += 1;
                }
            }
        }
        r
    }
}