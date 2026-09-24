#[derive(Clone)]
struct Node {
    max: i32,
    min: i32,
    add: i32,
}

struct SegTree {
    tree: Vec<Node>,
    n: usize,
}

impl SegTree {
    fn new(init_arr: &Vec<i32>) -> Self {
        let n = init_arr.len();
        let mut tree = vec![Node { max: 0, min: 0, add: 0 }; 4 * n];
        Self::build(&mut tree, init_arr, 0, 0, n - 1);
        SegTree { tree, n }
    }

    fn build(tree: &mut Vec<Node>, arr: &Vec<i32>, v: usize, tl: usize, tr: usize) {
        if tl == tr {
            tree[v] = Node { max: arr[tl], min: arr[tl], add: 0 };
        } else {
            let tm = (tl + tr) / 2;
            Self::build(tree, arr, 2 * v + 1, tl, tm);
            Self::build(tree, arr, 2 * v + 2, tm + 1, tr);
            tree[v].min = tree[2 * v + 1].min.min(tree[2 * v + 2].min);
            tree[v].max = tree[2 * v + 1].max.max(tree[2 * v + 2].max);
        }
    }

    fn push(&mut self, v: usize) {
        if self.tree[v].add != 0 {
            self.tree[2 * v + 1].max += self.tree[v].add;
            self.tree[2 * v + 1].min += self.tree[v].add;
            self.tree[2 * v + 1].add += self.tree[v].add;
            self.tree[2 * v + 2].max += self.tree[v].add;
            self.tree[2 * v + 2].min += self.tree[v].add;
            self.tree[2 * v + 2].add += self.tree[v].add;
            self.tree[v].add = 0;
        }
    }

    fn range_add(&mut self, ql: usize, qr: usize, add_val: i32) {
        self._range_add(0, 0, self.n - 1, ql, qr, add_val);
    }

    fn _range_add(&mut self, v: usize, tl: usize, tr: usize, ql: usize, qr: usize, add_val: i32) {
        if tr < ql || qr < tl {
            return;
        }
        if ql <= tl && tr <= qr {
            self.tree[v].max += add_val;
            self.tree[v].min += add_val;
            self.tree[v].add += add_val;
            return;
        }
        if tl < tr {
            self.push(v);
        }
        let tm = (tl + tr) / 2;
        if ql <= tm {
            self._range_add(2 * v + 1, tl, tm, ql, qr, add_val);
        }
        if tm < qr {
            self._range_add(2 * v + 2, tm + 1, tr, ql, qr, add_val);
        }
        self.tree[v].max = self.tree[2 * v + 1].max.max(self.tree[2 * v + 2].max);
        self.tree[v].min = self.tree[2 * v + 1].min.min(self.tree[2 * v + 2].min);
    }

    fn find_last(&mut self, ql: usize, qr: usize, val: i32) -> Option<usize> {
        self._find_last(0, 0, self.n - 1, ql, qr, val)
    }

    fn _find_last(&mut self, v: usize, tl: usize, tr: usize, ql: usize, qr: usize, val: i32) -> Option<usize> {
        if ql > tr || qr < tl || self.tree[v].min > val || self.tree[v].max < val {
            return None;
        }
        if tl == tr {
            return if self.tree[v].min == val { Some(tl) } else { None };
        }
        self.push(v);
        let tm = (tl + tr) / 2;
        let right_res = self._find_last(2 * v + 2, tm + 1, tr, ql, qr, val);
        if right_res.is_some() {
            return right_res;
        }
        return self._find_last(2 * v + 1, tl, tm, ql, qr, val);
    }
}

impl Solution {
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut v = nums.iter().map(|x| 1 - 2 * (x % 2)).collect::<Vec<_>>();
        let mut prev = vec![n; n];
        let mut last = vec![n; 100_001];
        let mut next = vec![n; n];

        for i in 0..n {
            let p = last[nums[i] as usize];
            if p < n {
                prev[i] = p;
                next[p] = i;
            }
            last[nums[i] as usize] = i;
        }
        let mut diff = vec![0; n];
        let mut curr = 0;
        for i in 0..n {
            if prev[i] == n {
                curr += v[i];
            }
            diff[i] = curr;
        }
        
        let mut seg_tree = SegTree::new(&diff);
        let mut max_len = 0;
        let mut offset = 0;
        for i in 0..n {
            if n - i <= max_len {
                break;
            }
            if let Some(j) = seg_tree.find_last(i, n - 1, offset) {
                max_len = max_len.max(j - i + 1);
            }
            if next[i] == n {
                offset += v[i];
            } else {
                seg_tree.range_add(i, next[i] - 1, -v[i]);
            }
        }
        max_len as _
    }
}