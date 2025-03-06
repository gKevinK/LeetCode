impl Solution {
    pub fn max_rectangle_area(x_coord: Vec<i32>, y_coord: Vec<i32>) -> i64 {
        use std::collections::HashMap;
        let mut ys = y_coord.clone();
        let mut points: Vec<_> = std::iter::zip(x_coord, y_coord).collect();

        points.sort_unstable();
        ys.sort_unstable();
        ys.dedup();
        let mut y_idx: HashMap<i32, usize> = HashMap::from_iter(ys.iter().enumerate().map(|(i, y)| (*y, i)));
        let mut y_max: HashMap<i32, i32>   = HashMap::from_iter(ys.iter().map(|y| (*y, i32::MIN)));
        let mut tree = Self::seg_tree_new((usize::BITS - ys.len().leading_zeros()) as usize);
        let (mut prev_x, mut prev_y) = points[0];
        let mut max_area = -1;

        for &(x, y) in points.iter().skip(1) {
            let q = Self::seg_tree_query(&tree, y_idx[&prev_y], y_idx[&y]);
            if prev_x == x && y_max[&prev_y] == y_max[&y] && y_max[&y] > q {
                max_area = max_area.max((y - prev_y) as i64 * (x - y_max[&y]) as i64);
            }

            y_max.get_mut(&prev_y).map(|val| { *val = prev_x; });
            Self::seg_tree_update(&mut tree, y_idx[&prev_y], prev_x);
            prev_x = x;
            prev_y = y;
        }

        max_area
    }

    fn seg_tree_new(n: usize) -> Vec<i32> {
        vec![i32::MIN; 1 << (n + 1)]
    }

    fn seg_tree_update(t: &mut Vec<i32>, mut idx: usize, val: i32) {
        idx += t.len() >> 1;
        while idx > 0 {
            t[idx] = val;
            idx >>= 1;
        }
    }

    fn seg_tree_query(t: &Vec<i32>, mut l: usize, mut r: usize) -> i32 {
        let mut res = i32::MIN;
        l += t.len() >> 1;
        r += t.len() >> 1;
        while r - l > 1 {
            if l & 1 == 0 {
                res = res.max(t[l + 1]);
            }
            if r & 1 == 1 {
                res = res.max(t[r - 1]);
            }
            l >>= 1;
            r >>= 1;
        }
        res
    }
}