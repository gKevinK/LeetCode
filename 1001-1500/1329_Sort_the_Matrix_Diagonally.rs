impl Solution {
    pub fn diagonal_sort(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::cmp::min;
        
        let m = mat.len();
        let n = mat[0].len();
        for r in 0..(m + n) {
            let x = if r < m { r } else { 0 };
            let y = if r >= m { r - m } else { 0 };
            let len = min(m - x, n - y);
            for _ in 0..(len - 1) {
                for i in 0..(len - 1) {
                    if mat[x + i][y + i] > mat[x + i + 1][y + i + 1] {
                        let t = mat[x + i][y + i];
                        mat[x + i][y + i] = mat[x + i + 1][y + i + 1];
                        mat[x + i + 1][y + i + 1] = t;
                    }
                }
            }
        }
        mat
    }
}