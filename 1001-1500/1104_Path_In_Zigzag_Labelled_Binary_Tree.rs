impl Solution {
    pub fn path_in_zig_zag_tree(label: i32) -> Vec<i32> {
        let mut res = vec![label];
        let mut t = 1;
        let mut curr = label;
        while t <= label {
            t <<= 1;
        }
        while curr != 1 {
            t >>= 1;
            let next = t - 1 - (curr - t) / 2;
            res.push(next);
            curr = next;
        }
        res.reverse();
        res
    }
}