impl Solution {
    pub fn min_cost(n: i32, mut edges: Vec<Vec<i32>>, k: i32) -> i32 {
        if n == k {
            return 0;
        }
        let mut es = edges.iter().map(|e| (e[2], e[0] as usize, e[1] as usize)).collect::<Vec<_>>();
        es.sort_unstable();
        let mut comp = n;
        let mut dsu = (0..n as usize).map(|x| (x, 0)).collect::<Vec<_>>();
        for &(w, u, v) in &es {
            if Self::find(&mut dsu, u) != Self::find(&mut dsu, v) {
                comp -= 1;
                if comp == k {
                    return w;
                }
                Self::unite(&mut dsu, u, v);
            }
        }
        0
    }

    fn find(mut dsu: &mut Vec<(usize, i32)>, i: usize) -> usize {
        if dsu[i].0 == i {
            return i;
        }
        let p = dsu[i].0;
        dsu[i].0 = Self::find(&mut dsu, p);
        dsu[i].0
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