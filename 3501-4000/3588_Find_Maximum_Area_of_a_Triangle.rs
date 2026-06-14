impl Solution {
    pub fn max_area(coords: Vec<Vec<i32>>) -> i64 {
        let n = coords.len();
        let mut points: Vec<_> = coords.iter().map(|c| (c[0], c[1])).collect();
        points.sort_unstable();
        let mut res = -1;

        let min_x = points[0].0;
        let max_x = points[n - 1].0;
        if min_x == max_x {
            return -1;
        }
        let mut i = 0;
        while i < n {
            let x = points[i].0;
            let mut j = i + 1;
            while j < n && points[j].0 == x {
                j += 1;
            }
            let len = points[j - 1].1 - points[i].1;
            if len > 0 {
                res = res.max(len as i64 * (x - min_x).max(max_x - x) as i64);
            }
            i = j;
        }
        points.sort_unstable_by_key(|a| (a.1, a.0));
        let min_y = points[0].1;
        let max_y = points[n - 1].1;
        if min_y == max_y {
            return -1;
        }
        i = 0;
        while i < n {
            let y = points[i].1;
            let mut j = i + 1;
            while j < n && points[j].1 == y {
                j += 1;
            }
            let len = points[j - 1].0 - points[i].0;
            if len > 0 {
                res = res.max(len as i64 * (y - min_y).max(max_y - y) as i64);
            }
            i = j;
        }

        res
    }
}