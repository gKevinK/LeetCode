impl Solution {
    pub fn good_subtree_sum(vals: Vec<i32>, par: Vec<i32>) -> i32 {
        let n = vals.len();
        let mut innum = vec![0; n];
        for &p in &par {
            if p >= 0 {
                innum[p as usize] += 1;
            }
        }
        let mut queue = Vec::with_capacity(n);
        queue.extend((0..n).filter(|i| innum[*i] == 0));
        let mut score = vec![vec![]; n];
        let init = vec![0; 1 << 10];
        let mut res = 0;
        let MOD = 1_000_000_007;
        let nl = queue.len();
        for ii in 0..n {
            let i = queue[ii];
            if ii >= nl {
                score[i].extend(&init);
                let m = Self::mask(vals[i]);
                if m > 0 {
                    score[i][m] = vals[i];
                }
            }
            if par[i] >= 0 {
                let p = par[i] as usize;
                innum[p] -= 1;
                if innum[p] == 0 {
                    queue.push(p);
                }
            }
        }

        let add = |mut score: &mut Vec<Vec<i32>>, p: usize, m1: usize, v: i32| {
            let mr = ((1 << 10) - 1) ^ m1;
            let mut m2 = mr;
            while m2 > 0 {
                if score[p][m2] > 0 {
                    score[p][m1 + m2] = score[p][m1 + m2].max((score[p][m2] + v) % MOD);
                }
                m2 = (m2 - 1) & mr;
            }
            score[p][m1] = score[p][m1].max(v);
        };

        for ii in 0..n {
            let i = queue[ii];
            if ii < nl {
                let m1 = Self::mask(vals[i]);
                if m1 > 0 {
                    res = (res + vals[i]) % MOD;
                    if par[i] >= 0 {
                        add(&mut score, par[i] as usize, m1, vals[i]);
                    }
                }
            } else {
                res = (res + *score[i].iter().max().unwrap()) % MOD;
                if par[i] < 0 {
                    continue;
                }
                let p = par[i] as usize;
                for m1 in 1..(1 << 10) {
                    if score[i][m1] > 0 {
                        let v = score[i][m1];
                        add(&mut score, p, m1, v);
                    }
                }
            }
        }
        res
    }

    fn mask(mut i: i32) -> usize {
        let mut m = 0;
        while i > 0 {
            let dm = 1 << (i % 10) as usize;
            if m & dm > 0 {
                return 0;
            }
            i /= 10;
            m += dm;
        }
        m
    }
}