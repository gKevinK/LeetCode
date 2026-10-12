impl Solution {
    pub fn max_subgraph_score(n: i32, edges: Vec<Vec<i32>>, good: Vec<i32>) -> Vec<i32> {
        let n = n as usize;
        let mut adj = vec![vec![]; n];
        for e in &edges {
            adj[e[0] as usize].push(e[1] as usize);
            adj[e[1] as usize].push(e[0] as usize);
        }
        let values = good.iter().map(|&x| 2 * x - 1).collect::<Vec<_>>();
        let mut dp = vec![0; n];
        let mut res = vec![0; n];

        Self::dfs_down(0, n, &adj, &values, &mut dp);
        res[0] = dp[0];
        Self::dfs_up(0, n, &adj, &dp, &mut res);
        res
    }

    fn dfs_down(u: usize, p: usize, adj: &Vec<Vec<usize>>, values: &Vec<i32>, mut dp: &mut Vec<i32>) {
        dp[u] = values[u];
        for &v in &adj[u] {
            if v == p {
                continue;
            }
            Self::dfs_down(v, u, &adj, &values, &mut dp);
            if dp[v] > 0 {
                dp[u] += dp[v];
            }
        }
    }

    fn dfs_up(u: usize, p: usize, adj: &Vec<Vec<usize>>, dp: &Vec<i32>, mut res: &mut Vec<i32>) {
        for &v in &adj[u] {
            if v == p {
                continue;
            }
            let score = res[u] - dp[v].max(0);
            res[v] = dp[v] + score.max(0);
            Self::dfs_up(v, u, &adj, &dp, &mut res);
        }
    }
}