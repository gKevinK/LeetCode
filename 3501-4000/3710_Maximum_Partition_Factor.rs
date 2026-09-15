impl Solution {
    pub fn max_partition_factor(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        if n <= 2 {
            return 0;
        }
        let mdist = |i: usize, j: usize| -> i32 {
            (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs()
        };
        let (mut md, mut mi, mut mj) = (i32::MAX, 0, 1);
        for i in 0..n {
            for j in (i + 1)..n {
                let dist = mdist(i, j);
                if dist < md {
                    md = dist;
                    (mi, mj) = (i, j);
                }
            }
        }
        let mut di = vec![0; n];
        let mut dj = vec![0; n];
        for x in 0..n {
            di[x] = mdist(mi, x);
            dj[x] = mdist(mj, x);
        }
        di[mi] = i32::MAX;
        di[mj] = i32::MAX;
        dj[mi] = i32::MAX;
        dj[mj] = i32::MAX;
        let mut res = i32::MAX;
        for _ in 0..n - 2 {
            let i = di.iter().enumerate().min_by_key(|&(_, &v)| v).unwrap().0;
            let j = dj.iter().enumerate().min_by_key(|&(_, &v)| v).unwrap().0;
            let idx = if di[i] < dj[j] { i } else { j };
            if di[idx] < dj[idx] {
                res = res.min(dj[idx]);
                for x in 0..n {
                    if dj[x] != i32::MAX {
                        dj[x] = dj[x].min(mdist(x, idx));
                    }
                }
            } else {
                res = res.min(di[idx]);
                for x in 0..n {
                    if di[x] != i32::MAX {
                        di[x] = di[x].min(mdist(x, idx));
                    }
                }
            }
            di[idx] = i32::MAX;
            dj[idx] = i32::MAX;
        }
        res
    }
}