use std::collections::BTreeSet;
#[derive(Debug)]
struct Status {
    sum_l: i64,
    sum_r: i64,
    set_l: BTreeSet<(i64, usize)>,
    set_r: BTreeSet<(i64, usize)>,
}

impl Status {
    fn new() -> Self {
        Status {
            sum_l: 0,
            sum_r: 0,
            set_l: BTreeSet::new(),
            set_r: BTreeSet::new(),
        }
    }

    fn balance(&mut self) {
        while self.set_l.len() + 1 < self.set_r.len() || self.set_r.first().unwrap_or(&(i64::MAX, 0)) < self.set_l.last().unwrap_or(&(0, 0)) {
            let mid = self.set_r.pop_first().unwrap();
            self.sum_r -= mid.0;
            self.sum_l += mid.0;
            self.set_l.insert(mid);
        }
        while self.set_l.len() > self.set_r.len() {
            let mid = self.set_l.pop_last().unwrap();
            self.sum_l -= mid.0;
            self.sum_r += mid.0;
            self.set_r.insert(mid);
        }
    }

    fn add(&mut self, x: (i64, usize)) {
        self.set_r.insert(x);
        self.sum_r += x.0;
        self.balance();
    }

    fn rm(&mut self, x: (i64, usize)) {
        if self.set_l.remove(&x) {
            self.sum_l -= x.0;
        } else if self.set_r.remove(&x) {
            self.sum_r -= x.0;
        }
        self.balance();
    }

    fn mid(&self) -> i64 {
        self.set_r.first().unwrap_or(&(-1, 0)).0
    }
}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32, queries: Vec<Vec<i32>>) -> Vec<i64> {
        let n = nums.len();
        let nq = queries.len();
        let xnums = nums.iter().map(|&num| (num / k) as i64).collect::<Vec<_>>();
        let mut z = vec![0; n];
        for i in 1..n {
            let diff = nums[i] - nums[i - 1];
            z[i] = z[i - 1] + i32::from(diff % k != 0);
        }

        let mut xqueries = vec![];
        for i in (0..nq) {
            let l = queries[i][0] as usize;
            let r = queries[i][1] as usize;
            if z[r] - z[l] > 0 {
                continue;
            }
            xqueries.push((i, l, r + 1));
        }
        let block = xqueries.len().isqrt() + 1;
        xqueries.sort_unstable_by_key(|&q| (q.1 / block, if q.1 / block % 2 == 0 { n + q.2 } else { n - q.2 }));

        let mut status = Status::new();
        let mut cl = 0;
        let mut cr = 0;
        let mut res = vec![-1; nq];
        for &(iq, l, r) in &xqueries {
            while cr < r {
                status.add((xnums[cr], cr));
                cr += 1;
            }
            while cl > l {
                cl -= 1;
                status.add((xnums[cl], cl));
            }
            while cr > r {
                cr -= 1;
                status.rm((xnums[cr], cr));
            }
            while cl < l {
                status.rm((xnums[cl], cl));
                cl += 1;
            }
            let mid = status.mid();
            res[iq] = mid * status.set_l.len() as i64 - status.sum_l + status.sum_r - mid * status.set_r.len() as i64;
        }
        res
    }
}