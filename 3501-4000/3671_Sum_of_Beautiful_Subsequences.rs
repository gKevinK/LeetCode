const MOD: i64 = 1_000_000_007;
const N: i64 = 70_000;

struct Fenwick {
    tree: Vec<i64>,
    n: usize,
}

impl Fenwick {
    fn new(n: usize) -> Self {
        Fenwick { tree: vec![0; n + 1], n }
    }

    fn add(&mut self, mut idx: usize, x: i64) {
        let x = (x % MOD + MOD) % MOD;
        while idx <= self.n {
            self.tree[idx] = (self.tree[idx] + x) % MOD;
            idx += idx & idx.wrapping_neg();
        }
    }

    fn query(&self, mut idx: usize) -> i64 {
        let mut sum = 0;
        while idx > 0 {
            sum = (sum + self.tree[idx]) % MOD;
            idx -= idx & idx.wrapping_neg();
        }
        sum
    }
}

impl Solution {
    pub fn total_beauty(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let max = *nums.iter().max().unwrap() as usize;
        let mut divs = vec![vec![]; max + 1];
        for d in 1..=max {
            for m in (d..=max).step_by(d) {
                divs[m].push(d);
            }
        }
        let mut groups = vec![vec![]; max + 1];
        for i in 0..n {
            for &d in &divs[nums[i] as usize] {
                groups[d].push(i);
            }
        }
        let mut sorted = nums.clone();
        sorted.sort();
        sorted.dedup();
        let m = sorted.len();
        let mut rank = vec![0; max as usize + 1];
        for (idx, &v) in sorted.iter().enumerate() {
            rank[v as usize] = idx + 1;
        }
        
        let mut tree = Fenwick::new(m);
        let mut phi = (0..=max).map(|x| x as i64).collect::<Vec<_>>();
        for i in 2..=max {
            if phi[i] == i as i64 {
                for j in (i..=max).step_by(i) {
                    phi[j] -= phi[j] / i as i64;
                }
            }
        }
        phi[1] = 1;
        let mut res = 0i64;
        for g in 1..=max {
            if groups[g].is_empty() {
                continue;
            }
            let mut updates = Vec::with_capacity(groups[g].len());
            for &idx in &groups[g] {
                let r = rank[nums[idx] as usize];
                let less = tree.query(r - 1);
                let delta = (less + 1) % MOD;
                tree.add(r, delta);
                updates.push((r, delta));
            }
            let total = updates.iter().fold(0i64, |acc, &(_, d)| (acc + d) % MOD);
            res = (res + phi[g] % MOD * total) % MOD;
            for (r, delta) in updates {
                tree.add(r, (MOD - delta) % MOD);
            }
        }
        res as i32
    }
}