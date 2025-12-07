impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        if k == 0 {
            return vec![1; edges1.len() + 1];
        }
        let add = *Self::one_graph(&edges2, k - 1).iter().max().unwrap();
        Self::one_graph(&edges1, k).iter().map(|i| i + add).collect()
    }

    fn one_graph(edges: &Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let n = edges.len() + 1;
        let r0 = vec![1; n];
        if k == 0 {
            return r0;
        }
        let mut g = vec![vec![]; n];
        for e in edges {
            g[e[0] as usize].push(e[1] as usize);
            g[e[1] as usize].push(e[0] as usize);
        }
        let mut rk2 = vec![0; n];
        let mut rk1 = r0;
        let mut rk = (0..n).map(|i| 1 + g[i].len() as i32).collect::<Vec<i32>>();
        for _ in 1..k {
            std::mem::swap(&mut rk2, &mut rk1);
            std::mem::swap(&mut rk1, &mut rk);
            for i in 0..n {
                rk[i] = g[i].iter().map(|e| rk1[*e]).sum::<i32>() - (g[i].len() as i32 - 1) * rk2[i];
            }
        }
        rk
    }
}