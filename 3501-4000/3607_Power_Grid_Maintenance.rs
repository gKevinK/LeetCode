impl Solution {
    pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::{HashMap, BinaryHeap};
        use std::cmp::Reverse;

        let n = c as usize;
        let mut dsu = (0..=n).map(|i| (i, 0)).collect::<Vec<_>>();
        for con in &connections {
            Self::unite(&mut dsu, con[0] as usize, con[1] as usize);
        }
        let mut grid = vec![BinaryHeap::new(); n + 1];
        for i in 1..=n {
            grid[Self::find(&mut dsu, i)].push(Reverse(i));
        }
        let mut online = vec![true; n + 1];
        let mut res = Vec::with_capacity(queries.len());
        for q in &queries {
            let q1 = q[1] as usize;
            let g = Self::find(&mut dsu, q1);
            if q[0] == 1 {
                if online[q1] {
                    res.push(q[1]);
                } else {
                    let mut heap = &mut grid[g];
                    while heap.peek().is_some_and(|&Reverse(x)| online[x] == false) {
                        heap.pop();
                    }
                    res.push(heap.peek().map_or(-1, |&Reverse(f)| f as i32));
                }
            } else {
                online[q1] = false;
            }
        }
        res
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
                dsu[ia].1 = dsu[ia].1 + (dsu[ib].1 + 1);
            } else {
                dsu[ia].0 = ib;
                dsu[ib].1 = dsu[ib].1 + (dsu[ia].1 + 1);
            }
        }
    }
}