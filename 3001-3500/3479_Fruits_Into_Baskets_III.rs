impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, mut baskets: Vec<i32>) -> i32 {
        let mut len = 1;
        let mut _n = fruits.len();
        while _n > 0 {
            _n >>= 1;
            len <<= 1;
        }
        let n = fruits.len();
        let mut tree = vec![0; len];
        for _i in 0..len {
            let mut i = _i;
            tree[i] = if i < n { baskets[i] } else { 0 };
            let mut mask = 1;
            while mask <= i {
                if i & mask > 0 {
                    tree[i] = tree[i].max(tree[i ^ mask]);
                }
                i = i | mask;
                mask <<= 1;
            }
        }
        let mut res = 0;
        for i in 0..n {
            if fruits[i] > tree[len - 1] {
                res += 1;
                continue;
            }
            let mut x = len - 1;
            let mut mask = len >> 1;
            while mask > 0 {
                if tree[x ^ mask] >= fruits[i] {
                    x = x ^ mask;
                }
                mask >>= 1;
            }
            tree[x] = 0;
            baskets[x] = 0;
            mask = 1;
            while mask < len {
                if x & mask > 0 {
                    tree[x] = tree[x].max(tree[x ^ mask]);
                    x = x | mask;
                    mask <<= 1;
                } else {
                    x = x | mask;
                    mask = 1;
                    tree[x] = if x < n { baskets[x] } else { 0 };
                }
            }
        }
        res
    }
}