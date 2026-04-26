struct SegTree {
    tree: Vec<(i32, [i32; 5])>,
    n: usize,
    k: i32,
    ku: usize,
}

impl SegTree {
    fn new(k: i32, arr: &Vec<i32>) -> Self {
        let n = arr.len();
        let mut stree = SegTree {
            tree: vec![(0, [0; 5]); 4 * n],
            n: n,
            k: k,
            ku: k as usize,
        };
        stree.build(&arr, 0, 0, n - 1);
        stree
    }

    fn build(&mut self, arr: &Vec<i32>, v: usize, tl: usize, tr: usize) {
        if tl == tr {
            let val = arr[tl] % self.k;
            self.tree[v].0 = val;
            self.tree[v].1[val as usize] = 1;
        } else {
            let tm = (tl + tr) / 2;
            self.build(&arr, v * 2 + 1, tl, tm);
            self.build(&arr, v * 2 + 2, tm + 1, tr);
            self.merge(v);
        }
    }

    fn merge(&mut self, v: usize) {
        let l = v * 2 + 1;
        let r = v * 2 + 2;
        self.tree[v].0 = (self.tree[l].0 * self.tree[r].0) % self.k;
        for i in 0..self.ku {
            self.tree[v].1[i] = self.tree[l].1[i];
        }
        for i in 0..self.ku {
            let rem = (self.tree[l].0 as usize * i) % self.ku;
            self.tree[v].1[rem] += self.tree[r].1[i];
        }
    }

    fn update(&mut self, v: usize, tl: usize, tr: usize, target: usize, val: i32) {
        if tl == tr {
            self.tree[v].0 = val;
            self.tree[v].1.fill(0);
            self.tree[v].1[val as usize] = 1;
        } else {
            let tm = (tl + tr) / 2;
            if target <= tm {
                self.update(v * 2 + 1, tl, tm, target, val);
            } else {
                self.update(v * 2 + 2, tm + 1, tr, target, val);
            }
            self.merge(v);
        }
    }

    fn query(&self, v: usize, tl: usize, tr: usize, ql: usize, qr: usize) -> (i32, [i32; 5]) {
        if qr < tl || tr < ql {
            return (1, [0; 5]);
        }
        if ql <= tl && tr <= qr {
            return self.tree[v];
        }
        let tm = (tl + tr) / 2;
        let mut xl = self.query(v * 2 + 1, tl, tm, ql, qr);
        let xr = self.query(v * 2 + 2, tm + 1, tr, ql, qr);
        for i in 0..self.ku {
            xl.1[(xl.0 as usize * i) % self.ku] += xr.1[i];
        }
        xl.0 = (xl.0 * xr.0) % self.k;
        xl
    }
}

impl Solution {
    pub fn result_array(mut nums: Vec<i32>, k: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        let mut stree = SegTree::new(k, &nums);
        if k == 1 {
            return queries.iter().map(|q| n as i32 - q[2]).collect();
        }

        let mut result = vec![0; queries.len()];
        for (iq, query) in queries.iter().enumerate() {
            stree.update(0, 0, n - 1, query[0] as usize, query[1] % k);
            let node = stree.query(0, 0, n - 1, query[2] as usize, n - 1);
            result[iq] = node.1[query[3] as usize];
        }
        result
    }
}