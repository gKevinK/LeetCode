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
    
    fn range_sum(&self, l: usize, r: usize) -> i32 {
        if l > r { 0 } else { self.sum(r) - self.sum(l - 1) }
    }
}

impl Solution {
    pub fn min_inversion_count(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let ku = k as usize;
        if k <= 1 {
            return 0;
        }
        let mut sorted = nums.clone();
        sorted.sort_unstable();
        sorted.dedup();
        let m = sorted.len();
        let mut tree = FenwickTree::new(m);

        let mut inv = 0;
        for i in 0..ku {
            let pos = sorted.binary_search(&nums[i]).unwrap();
            inv += tree.range_sum(pos + 1, m - 1) as i64;
            tree.add(pos, 1);
        }
        let mut res = inv;
        for i in 0..(n - ku) {
            let pos_rm = sorted.binary_search(&nums[i]).unwrap();
            inv -= tree.range_sum(0, pos_rm - 1) as i64;
            tree.add(pos_rm, -1);
            let pos_add = sorted.binary_search(&nums[i + ku]).unwrap();
            inv += tree.range_sum(pos_add + 1, m - 1) as i64;
            tree.add(pos_add, 1);
            res = res.min(inv);
        }
        res
    }
}