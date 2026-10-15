struct FenwickTree {
    tree: Vec<i32>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        FenwickTree {
            tree: vec![0; n + 2],
        }
    }
    
    fn add(&mut self, mut idx: usize, delta: i32) {
        idx += 1;
        while idx < self.tree.len() {
            self.tree[idx] += delta;
            idx += idx & (!idx + 1);
        }
    }
    
    fn sum(&self, mut idx: usize) -> i32 {
        idx += 1;
        let mut res = 0;
        while idx > 0 {
            res += self.tree[idx];
            idx -= idx & (!idx + 1);
        }
        res
    }
}

impl Solution {
    pub fn min_deletions(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut bytes = s.bytes().collect::<Vec<_>>();
        let n = bytes.len();
        let mut tree = FenwickTree::new(n);
        for i in 1..n {
            if bytes[i - 1] == bytes[i] {
                tree.add(i, 1);
            }
        }
        let mut res = Vec::with_capacity(queries.len());
        for (iq, q) in queries.iter().enumerate() {
            if q[0] == 1 {
                let j = q[1] as usize;
                bytes[j] = b'B' - (bytes[j] - b'A');
                if j > 0 {
                    tree.add(j, i32::from(bytes[j - 1] == bytes[j]) * 2 - 1);
                }
                if j + 1 < n {
                    tree.add(j + 1, i32::from(bytes[j] == bytes[j + 1]) * 2 - 1);
                }
            } else {
                let l = q[1] as usize;
                let r = q[2] as usize;
                res.push(tree.sum(r) - tree.sum(l));
            }
        }
        res
    }
}