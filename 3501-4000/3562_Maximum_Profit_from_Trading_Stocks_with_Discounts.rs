impl Solution {
    pub fn max_profit(n: i32, present: Vec<i32>, future: Vec<i32>, hierarchy: Vec<Vec<i32>>, budget: i32) -> i32 {
        let nu = n as usize;
        let mut sub = vec![vec![]; nu + 1];
        let mut boss = vec![0; nu + 1];
        let mut innum = vec![0; nu + 1];
        for h in &hierarchy {
            sub[h[0] as usize].push(h[1] as usize);
            boss[h[1] as usize] = h[0] as usize;
            innum[h[0] as usize] += 1;
        }
        let mut order = Vec::with_capacity(nu);
        order.extend((1..=nu).filter(|x| innum[*x] == 0));
        for i in 0..(nu - 1) {
            let b = boss[order[i]];
            innum[b] -= 1;
            if innum[b] == 0 {
                order.push(b);
            }
        }
        let bu = budget as usize;
        let MIN = i32::MIN / 2;
        let mut dp = vec![(vec![MIN; bu + 1], vec![MIN; bu + 1]); nu + 1];
        let mut next = (vec![MIN; bu + 1], vec![MIN; bu + 1]);
        for i in order {
            next.0[0] = 0;
            next.1[0] = 0;
            for &isub in &sub[i] {
                let sub_p = present[isub - 1];
                let sub_f = future[isub - 1];
                for b1 in (0..=bu).rev() {
                    if next.1[b1] >= 0 {
                        for b2 in (0..=(bu - b1)).rev() {
                            let sum = b1 + b2 + sub_p as usize / 2;
                            if sum <= bu && dp[isub].1[b2] >= 0 {
                                next.1[sum] = next.1[sum].max(next.1[b1] + dp[isub].1[b2] + sub_f - sub_p / 2);
                            }
                            if dp[isub].0[b2] >= 0 {
                                next.1[b1 + b2] = next.1[b1 + b2].max(next.1[b1] + dp[isub].0[b2]);
                            }
                        }
                    }
                    if next.0[b1] >= 0 {
                        for b2 in (0..=(bu - b1)).rev() {
                            let sum = b1 + b2 + sub_p as usize;
                            if sum <= bu && dp[isub].1[b2] >= 0 {
                                next.0[sum] = next.0[sum].max(next.0[b1] + dp[isub].1[b2] + sub_f - sub_p);
                            }
                            if dp[isub].0[b2] >= 0 {
                                next.0[b1 + b2] = next.0[b1 + b2].max(next.0[b1] + dp[isub].0[b2]);
                            }
                        }
                    }
                }
            }
            std::mem::swap(&mut dp[i], &mut next);
        }
        let mut res = 0;
        let p0 = present[0];
        let f0 = future[0];
        for i in 0..=bu {
            res = res.max(dp[1].0[i]);
            if bu - i >= p0 as usize {
                res = res.max(dp[1].1[i] + f0 - p0);
            }
        }
        res
    }
}