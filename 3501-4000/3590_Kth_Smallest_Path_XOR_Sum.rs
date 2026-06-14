#[derive(Clone, Debug)]
struct TNode {
    num: usize,
    child: [usize; 16],
}

#[derive(Clone)]
struct Tree {
    nodes: Vec<TNode>,
}

impl Default for Tree {
    fn default() -> Tree {
        Tree::new()
    }
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            nodes: vec![TNode { num: 0, child: [0; 16] }, TNode { num: 0, child: [0; 16] }],
        }
    }

    pub fn len(&self) -> usize {
        self.nodes[0].num + self.nodes[1].num
    }

    pub fn insert(&mut self, i: usize) {
        let mut r = i >> 16;
        for l in 0..3 {
            let c = (i >> 4 * (3 - l)) & 0b1111;
            if self.nodes[r].child[c] == 0 {
                self.nodes[r].child[c] = self.nodes.len();
                self.nodes.push(TNode { num: 0, child: [0; 16] });
            }
            r = self.nodes[r].child[c];
        }
        let c = i & 0b1111;
        if self.nodes[r].child[c] == 0 {
            self.nodes[r].child[c] = 1;
            r = i >> 16;
            self.nodes[r].num += 1;
            for l in 0..3 {
                let c = (i >> 4 * (3 - l)) & 0b1111;
                r = self.nodes[r].child[c];
                self.nodes[r].num += 1;
            }
        }

    }

    pub fn nth(&self, mut n: usize) -> usize {
        let mut res = 0;
        let mut r = 0;
        if self.nodes[0].num <= n {
            res = 1 << 16;
            r = 1;
            n -= self.nodes[0].num;
        }
        for l in 0..3 {
            for c in 0..16 {
                let r2 = self.nodes[r].child[c];
                if r2 > 1 {
                    if self.nodes[r2].num <= n {
                        n -= self.nodes[r2].num;
                    } else {
                        res += c << 4 * (3 - l);
                        r = r2;
                        break;
                    }
                }
            }
        }
        let mut x = 0;
        let node = &self.nodes[r];
        for c in 0..16 {
            if node.child[c] == 1 {
                if x == n {
                    return res + c;
                }
                x += 1;
            }
        }
        i32::MAX as usize
    }

    pub fn merge(&mut self, other: &Self) {
        self.merge_dfs(&other, 0, 0, 0);
        self.merge_dfs(&other, 1, 1, 0);
    }

    fn merge_dfs(&mut self, other: &Self, r1: usize, r2: usize, l: usize) {
        if l == 3 {
            for c in 0..16 {
                self.nodes[r1].child[c] |= other.nodes[r2].child[c];
            }
            self.nodes[r1].num = self.nodes[r1].child.iter().sum();
        } else {
            self.nodes[r1].num = 0;
            for c in 0..16 {
                if other.nodes[r2].child[c] != 0 {
                    if self.nodes[r1].child[c] == 0 {
                        self.nodes[r1].child[c] = self.nodes.len();
                        self.nodes.push(TNode { num: 0, child: [0; 16] });
                    }
                    self.merge_dfs(&other, self.nodes[r1].child[c], other.nodes[r2].child[c], l + 1);
                }
                if self.nodes[r1].child[c] > 0 {
                    self.nodes[r1].num += self.nodes[self.nodes[r1].child[c]].num;
                }
            }
        }
    }
}

impl Solution {
    pub fn kth_smallest(par: Vec<i32>, vals: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = par.len();
        let mut out = vec![0; n];
        for i in 1..n {
            out[par[i] as usize] += 1;
        }
        let mut order = Vec::with_capacity(n);
        order.extend((0..n).filter(|i| out[*i] == 0));
        for i in 0..(n - 1) {
            let p = par[order[i]] as usize;
            out[p] -= 1;
            if out[p] == 0 {
                order.push(p);
            }
        }

        let mut has_q = par.iter().map(|p| *p as usize).collect::<Vec<_>>();
        let mut qs = vec![vec![]; n];
        has_q[0] = 0;
        for (i, q) in queries.iter().enumerate() {
            let u = q[0] as usize;
            has_q[u] = u;
            qs[u].push((q[1] as usize - 1, i));
        }
        let mut xor = vec![0; n];
        let mut cands = vec![None; n];
        for &i in order.iter().rev() {
            let p = par[i] as usize;
            let xp = if i != 0 { xor[p] } else { 0 };
            xor[i] = xp ^ vals[i];
            if has_q[i] != i {
                has_q[i] = has_q[p];
            }
        }
        let mut res = vec![-1; queries.len()];
        for &i in &order {
            cands[has_q[i]].get_or_insert(Tree::new()).insert(xor[i] as usize);
            if has_q[i] == i {
                let tree = cands[i].get_or_insert_default();
                for &q in &qs[i] {
                    if q.0 < tree.len() {
                        res[q.1] = tree.nth(q.0) as i32;
                    }
                }
                if i != 0 {
                    let next = has_q[par[i] as usize];
                    if cands[i].get_or_insert_default().len() < cands[next].get_or_insert_default().len() {
                        let x = cands[i].take().unwrap();
                        cands[next].get_or_insert_default().merge(&x);
                    } else {
                        let x = cands[next].take().unwrap();
                        cands[next] = cands[i].take();
                        cands[next].get_or_insert_default().merge(&x);
                    }
                }
            }
        }
        res
    }
}