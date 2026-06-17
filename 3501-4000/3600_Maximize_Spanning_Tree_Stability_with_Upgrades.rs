impl Solution {
    pub fn max_stability(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        let ku = k as usize;
        let mut dsu = (0..n as usize).map(|x| (x, 0)).collect::<Vec<_>>();
        let mut min = i32::MAX;
        let mut added = 0;
        let mut not_must = vec![];
        for e in &edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            if e[3] == 1 {
                if Self::find(&mut dsu, u) == Self::find(&mut dsu, v) {
                    return -1;
                }
                added += 1;
                min = min.min(e[2]);
                Self::unite(&mut dsu, u, v);
            } else {
                not_must.push((e[2], u, v));
            }
        }
        if added == n - 1 {
            return min;
        }
        not_must.sort_unstable();
        let mut add = std::collections::VecDeque::new();
        for i in (0..not_must.len()).rev() {
            let (s, u, v) = not_must[i];
            if Self::find(&mut dsu, u) != Self::find(&mut dsu, v) {
                added += 1;
                Self::unite(&mut dsu, u, v);
                add.push_back(s);
                if add.len() > ku {
                    min = min.min(add.pop_front().unwrap());
                }
            }
            if added == n - 1 {
                break;
            }
        }
        if added < n - 1 {
            return -1;
        }
        std::cmp::min(min, add.pop_back().unwrap_or(1000000) * 2)
    }

    fn find(mut dsu: &mut Vec<(usize, i32)>, i: usize) -> usize {
        let mut r = i;
        while dsu[r].0 != r {
            r = dsu[r].0;
        }
        dsu[i].0 = r;
        r
    }

    fn unite(mut dsu: &mut Vec<(usize, i32)>, a: usize, b: usize) {
        let ia = Self::find(&mut dsu, a);
        let ib = Self::find(&mut dsu, b);
        if ia != ib {
            if dsu[ia].1 > dsu[ib].1 {
                dsu[ib].0 = ia;
                dsu[ia].1 = dsu[ia].1.max(dsu[ib].1 + 1);
            } else {
                dsu[ia].0 = ib;
                dsu[ib].1 = dsu[ib].1.max(dsu[ia].1 + 1);
            }
        }
    }
}