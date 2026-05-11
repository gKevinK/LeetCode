impl Solution {
    pub fn subtree_inversion_sum(edges: Vec<Vec<i32>>, nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let k = k as usize;
        let mut g = vec![vec![]; n];
        for e in &edges {
            g[e[0] as usize].push(e[1] as usize);
            g[e[1] as usize].push(e[0] as usize);
        }
        let mut parent = vec![usize::MAX; n];
        let mut order = vec![];
        let mut stack = vec![0];
        parent[0] = 0;
        while let Some(i) = stack.pop() {
            order.push(i);
            for &v in &g[i] {
                if v != parent[i] {
                    parent[v] = i;
                    stack.push(v);
                }
            }
        }

        let mut dp = vec![vec![(0, 0); n]; k + 1];
        for &i in order.iter().rev() {
            for ik in 1..=k {
                let mut max = nums[i] as i64;
                let mut min = nums[i] as i64;
                for &v in &g[i] {
                    if v != parent[i] {
                        max += dp[k.min(ik + 1)][v].0;
                        min += dp[k.min(ik + 1)][v].1;
                    }
                }
                if ik == k {
                    let mut max_inv = -nums[i] as i64;
                    let mut min_inv = -nums[i] as i64;
                    for &v in &g[i] {
                        if v != parent[i] {
                            max_inv -= dp[1][v].1;
                            min_inv -= dp[1][v].0;
                        }
                    }
                    max = max.max(max_inv);
                    min = min.min(min_inv);
                }
                dp[ik][i] = (max, min);
            }
        }
        dp[k][0].0
    }
}