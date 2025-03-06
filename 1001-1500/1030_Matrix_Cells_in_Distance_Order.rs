impl Solution {
    pub fn all_cells_dist_order(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;
        
        let ds = [[0, 1], [1, 0], [-1, 0], [0, -1]];
        let mut q: VecDeque<(i32, i32)> = VecDeque::new();
        q.push_back((r0, c0));
        let mut dp: Vec<Vec<bool>> = vec![vec![false; c as usize]; r as usize];
        let mut res: Vec<Vec<i32>> = Vec::new();
        dp[r0 as usize][c0 as usize] = true;
        while let Some((r1, c1)) = q.pop_front() {
            res.push(vec![r1, c1]);
            for i in 0..4 {
                let r2 = r1 + ds[i][0];
                let c2 = c1 + ds[i][1];
                if r2 < 0 || r2 >= r || c2 < 0 || c2 >= c || dp[r2 as usize][c2 as usize] == true {
                    continue;
                }
                dp[r2 as usize][c2 as usize] = true;
                q.push_back((r2, c2));
            }
        }
        res
    }
}