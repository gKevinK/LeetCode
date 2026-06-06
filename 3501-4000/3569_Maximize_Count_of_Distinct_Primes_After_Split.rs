const LIMIT: usize = 100000;
const PRIME: [bool; LIMIT + 1] = {
    let mut p = [true; LIMIT + 1];
    p[0] = false;
    p[1] = false;
    let mut i = 2;
    let mut end = LIMIT.isqrt();
    while i <= end {
        if p[i] {
            let mut j = i * 2;
            while j <= LIMIT {
                p[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    p
};

#[derive(Clone)]
struct Node {
    val: i32,
    add: i32,
}

struct LazySegmentTree {
    tree: Vec<Node>,
    n: usize,
}

impl LazySegmentTree {
    fn new(init_arr: &Vec<i32>) -> Self {
        let n = init_arr.len();
        let mut tree = vec![Node { val: 0, add: 0 }; 4 * n];
        Self::build(&mut tree, init_arr, 0, 0, n - 1);
        LazySegmentTree { tree, n }
    }

    fn build(tree: &mut Vec<Node>, arr: &Vec<i32>, v: usize, tl: usize, tr: usize) {
        if tl == tr {
            tree[v] = Node { val: arr[tl], add: 0 };
        } else {
            let tm = (tl + tr) / 2;
            Self::build(tree, arr, 2 * v + 1, tl, tm);
            Self::build(tree, arr, 2 * v + 2, tm + 1, tr);
            tree[v].val = tree[2 * v + 1].val.max(tree[2 * v + 2].val);
        }
    }

    fn update(&mut self, ql: usize, qr: usize, add_val: i32) {
        self._update(0, 0, self.n - 1, ql, qr, add_val);
    }

    fn _update(&mut self, v: usize, tl: usize, tr: usize, ql: usize, qr: usize, add_val: i32) {
        if ql <= tl && qr >= tr {
            self.tree[v].add += add_val;
            self.tree[v].val += add_val;
            return;
        }
        let vl = 2 * v + 1;
        let vr = 2 * v + 2;
        let tm = (tl + tr) / 2;
        if self.tree[v].add != 0 && tl < tr {
            self.tree[vl].val += self.tree[v].add;
            self.tree[vl].add += self.tree[v].add;
            self.tree[vr].val += self.tree[v].add;
            self.tree[vr].add += self.tree[v].add;
            self.tree[v].add = 0;
        }
        if ql <= tm {
            self._update(vl, tl, tm, ql, qr, add_val);
        }
        if tm < qr {
            self._update(vr, tm + 1, tr, ql, qr, add_val);
        }
        self.tree[v].val = self.tree[vl].val.max(self.tree[vr].val);
    }

    fn query(&self) -> i32 {
        self.tree[0].val
    }
}

impl Solution {
    pub fn maximum_count(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        let mut idx = std::collections::HashMap::new();
        for i in 0..n {
            if PRIME[nums[i] as usize] {
                idx.entry(nums[i]).or_insert(std::collections::BTreeSet::new()).insert(i);
            }
        }
        let mut delta = vec![0; n + 1];
        for (p, set) in &idx {
            delta[*set.first().unwrap() + 1] += 1;
            delta[*set.last().unwrap() + 1] -= 1;
        }
        for i in 1..=n {
            delta[i] += delta[i - 1];
        }
        let mut tree = LazySegmentTree::new(&delta);
        let mut res = vec![0; queries.len()];
        for iq in 0..queries.len() {
            let i = queries[iq][0] as usize;
            let new_val = queries[iq][1];
            let old_val = nums[i];
            nums[i] = new_val;
            // Delete old value
            if PRIME[old_val as usize] {
                let mut set = idx.get_mut(&old_val).unwrap();
                if set.len() >= 2 {
                    let l = *set.first().unwrap();
                    let r = *set.last().unwrap();
                    if i == l || i == r {
                        tree.update(l + 1, r, -1);
                        set.remove(&i);
                        if set.len() >= 2 {
                            let l = *set.first().unwrap();
                            let r = *set.last().unwrap();
                            tree.update(l + 1, r, 1);
                        }
                    } else {
                        set.remove(&i);
                    }
                } else {
                    idx.remove(&old_val);
                }
            }
            // Add new value
            if PRIME[new_val as usize] {
                if let Some(set) = idx.get_mut(&new_val) {
                    let l = *set.first().unwrap();
                    let r = *set.last().unwrap();
                    if i < l || r < i {
                        if set.len() >= 2 {
                            tree.update(l + 1, r, -1);
                        }
                        set.insert(i);
                        let l = *set.first().unwrap();
                        let r = *set.last().unwrap();
                        tree.update(l + 1, r, 1);
                    } else {
                        set.insert(i);
                    }
                } else {
                    idx.insert(new_val, std::collections::BTreeSet::from([i]));
                }
            }
            res[iq] = tree.query() + idx.len() as i32;
        }
        res
    }
}