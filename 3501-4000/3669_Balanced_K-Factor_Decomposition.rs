impl Solution {
    pub fn min_difference(n: i32, k: i32) -> Vec<i32> {
        if k == 1 {
            return vec![n];
        }
        let mut div = vec![];
        for i in 1..=n.isqrt() {
            if n % i == 0 {
                div.push(i);
                if i * i != n {
                    div.push(n / i);
                }
            }
        }
        div.sort_unstable();
        let ku = k as usize;
        let mut state = Vec::with_capacity(ku);
        let mut res = vec![1; ku];
        res[ku - 1] = n;
        Self::dfs(0, n, ku, &div, &mut state, &mut res);
        res
    }

    fn dfs(x: usize, n: i32, k: usize, div: &Vec<i32>, mut state: &mut Vec<i32>, mut res: &mut Vec<i32>) {
        if state.len() >= 2 && state[state.len() - 1] - state[0] >= res[res.len() - 1] - res[0] {
            return;
        }
        if k == 1 {
            if n >= div[x] {
                state.push(n);
                let diff = state[state.len() - 1] - state[0];
                if diff < res[res.len() - 1] - res[0] {
                    res.copy_from_slice(&state);
                }
                state.pop();
            }
            return;
        }
        for i in x..div.len() {
            if n % div[i] == 0 {
                state.push(div[i]);
                Self::dfs(i, n / div[i], k - 1, &div, &mut state, &mut res);
                state.pop();
            }
        }
    }
}