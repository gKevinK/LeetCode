impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {
        let mut p1 = Self::parity(&edges1);
        let p2 = Self::parity(&edges2);

        let t1_o: i32 = p1.iter().sum();
        let t1_e = p1.len() as i32 - t1_o;
        let t2_o: i32 = p2.iter().sum();
        let t2_e = p2.len() as i32 - t2_o;
        let t2_max = std::cmp::max(t2_o, t2_e);

        for i in 0..p1.len() {
            p1[i] = if p1[i] == 1 { t1_o + t2_max } else { t1_e + t2_max };
        }
        p1
    }

    fn parity(edges: &Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len() + 1;
        let mut nodes = (0..n).map(|i| (i, 0, 0)).collect::<Vec<_>>();
        for e in edges {
            let (mut u, mut v) = (e[0] as usize, e[1] as usize);
            let (mut r1, o1) = Self::root(&nodes, u);
            let (mut r2, o2) = Self::root(&nodes, v);
            if nodes[u].0 != r1 {
                nodes[u] = (r1, o1, 0);
            }
            if nodes[v].0 != r2 {
                nodes[v] = (r2, o2, 0);
            }
            if nodes[r1].2 < nodes[r2].2 {
                (r1, r2) = (r2, r1);
            }
            nodes[r2].0 = r1;
            nodes[r2].1 = o1 ^ o2 ^ 1;
            nodes[r1].2 = nodes[r1].2.max(nodes[r2].2 + 1);
        }
        (0..n).map(|i| Self::root(&nodes, i).1).collect()
    }

    fn root(nodes: &Vec<(usize, i32, i32)>, i: usize) -> (usize, i32) {
        if nodes[i].0 != i {
            let (r, odd) = Self::root(&nodes, nodes[i].0);
            (r, odd ^ nodes[i].1)
        } else {
            (i, 0)
        }
    }
}