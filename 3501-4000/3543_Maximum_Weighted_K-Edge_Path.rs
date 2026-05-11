impl Solution {
    pub fn max_weight(n: i32, edges: Vec<Vec<i32>>, k: i32, t: i32) -> i32 {
        if k == 0 {
            return 0;
        }
        let nu = n as usize;
        let mut g = vec![vec![]; nu];
        for e in &edges {
            g[e[0] as usize].push((e[1] as usize, e[2]));
        }
        let mut dp = vec![vec![]; nu];
        for i in 0..nu {
            let x = Self::dfs(&g, &mut dp, k, t, i);
        }
        let mut res = -1;
        for i in 0..nu {
            for j in 0..dp[i].len() {
                let (len, sum) = dp[i][j];
                if len == k && sum < t {
                    res = res.max(sum);
                }
            }
        }
        res
    }

    fn dfs(g: &Vec<Vec<(usize, i32)>>, mut dp: &mut Vec<Vec<(i32, i32)>>, k: i32, t: i32, i: usize) {
        if g[i].len() == 0 || dp[i].len() > 0 {
            return;
        }
        let mut set = std::collections::HashSet::new();
        for j in 0..g[i].len() {
            let (v, w) = g[i][j];
            if w < t {
                dp[i].push((1, w));
            }
            Self::dfs(&g, &mut dp, k, t, v);
            for x in 0..dp[v].len() {
                let (len, sum) = dp[v][x];
                if len + 1 <= k && w + sum < t {
                    set.insert((len + 1, w + sum));
                }
            }
        }
        for x in set {
            dp[i].push(x);
        }
        if dp[i].len() == 0 {
            dp[i].push((k, t));
        }
    }
}