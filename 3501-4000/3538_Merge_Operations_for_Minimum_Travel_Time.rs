impl Solution {
    pub fn min_travel_time(l: i32, n: i32, k: i32, position: Vec<i32>, time: Vec<i32>) -> i32 {
        let nu = n as usize;
        let ku = k as usize;
        let mut dp = vec![vec![vec![]; nu]; ku + 1];
        dp[0][0].push((time[0], 0));
        let MAX = 1_000_000_000;
        for ik in 0..=ku {
            for i in 0..nu {
                if i + 1 < nu {
                    let dist = position[i + 1] - position[i];
                    let mut ncost = MAX;
                    for &(t, cost) in &dp[ik][i] {
                        ncost = ncost.min(cost + t * dist);
                    }
                    dp[ik][i + 1].push((time[i + 1], ncost));
                }
                if i + 2 < nu {
                    let mut dist = position[i + 1] - position[i];
                    let mut nt = time[i + 1];
                    let mut num = 1;
                    while ik + num <= ku && i + num + 1 < nu {
                        dist += position[i + num + 1] - position[i + num];
                        nt += time[i + num + 1];
                        let mut ncost = MAX;
                        for &(t, cost) in &dp[ik][i] {
                            ncost = ncost.min(cost + t * dist);
                        }
                        dp[ik + num][i + num + 1].push((nt, ncost));
                        num += 1;
                    }
                }
            }
        }
        dp[ku][nu - 1].iter().map(|x| x.1).min().unwrap()
    }
}