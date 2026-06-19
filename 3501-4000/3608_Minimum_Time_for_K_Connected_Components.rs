impl Solution {
    pub fn min_time(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut es = edges.iter().map(|e| (e[2], e[0] as usize, e[1] as usize)).collect::<Vec<_>>();
        es.sort_unstable();
        let mut comp = n;
        let mut dsu = (0..n as usize).map(|x| (x, 0)).collect::<Vec<_>>();
        for &(time, u, v) in es.iter().rev() {
            if Self::find(&mut dsu, u) != Self::find(&mut dsu, v) {
                comp -= 1;
                if comp < k {
                    return time;
                }
                Self::unite(&mut dsu, u, v);
            }
        }
        0
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