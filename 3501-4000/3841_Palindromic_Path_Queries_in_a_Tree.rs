impl Solution {
    pub fn palindrome_path(n: i32, edges: Vec<Vec<i32>>, s: String, queries: Vec<String>) -> Vec<bool> {
        let n = n as usize;
        let mut adj = vec![vec![]; n];
        for e in &edges {
            adj[e[0] as usize].push(e[1] as usize);
            adj[e[1] as usize].push(e[0] as usize);
        }
        const LOG: usize = 16;
        let mut ti = vec![0; n];
        let mut to = vec![0; n];
        let mut depth = vec![0; n];
        let mut up = vec![vec![0; n]; LOG + 1];
        let mut timer = 0usize;

        let mut stack = vec![];
        stack.push((0, 0, false));
        while let Some((node, parent, leave)) = stack.pop() {
            timer += 1;
            if leave {
                to[node] = timer;
            } else {
                ti[node] = timer;
                up[0][node] = parent;
                stack.push((node, parent, true));
                for &child in adj[node].iter().rev() {
                    if child != parent {
                        depth[child] = depth[node] + 1;
                        stack.push((child, node, false));
                    }
                }
            }
        }
        for k in 1..=LOG {
            for u in 0..n {
                up[k][u] = up[k - 1][up[k - 1][u]];
            }
        }

        let mut node_bit = s.bytes().map(|b| 1u32 << (b - b'a')).collect::<Vec<_>>();
        let bit_size = 2 * n + 2;
        let mut bit = vec![0u32; bit_size];

        for u in 0..n {
            let b = node_bit[u];
            let mut i = ti[u];
            while i < bit_size {
                bit[i] ^= b;
                i += i & (!i + 1);
            }
            let mut i = to[u];
            while i < bit_size {
                bit[i] ^= b;
                i += i & (!i + 1);
            }
        }

        let mut res = vec![];
        for q in &queries {
            let parts = q.split_whitespace().collect::<Vec<_>>();
            if parts[0] == "update" {
                let u = parts[1].parse::<usize>().unwrap();
                let c = parts[2].bytes().next().unwrap();
                let new_bit = 1u32 << (c - b'a');
                let delta = node_bit[u] ^ new_bit;
                if delta != 0 {
                    let mut i = ti[u];
                    while i < bit_size {
                        bit[i] ^= delta;
                        i += i & (!i + 1);
                    }
                    let mut i = to[u];
                    while i < bit_size {
                        bit[i] ^= delta;
                        i += i & (!i + 1);
                    }
                    node_bit[u] = new_bit;
                }
            } else {
                let u = parts[1].parse::<usize>().unwrap();
                let v = parts[2].parse::<usize>().unwrap();
                let (mut uu, mut vv) = (u, v);
                if depth[uu] < depth[vv] {
                    (uu, vv) = (vv, uu);
                }
                let diff = depth[uu] - depth[vv];
                for k in (0..=LOG).rev() {
                    if diff & (1 << k) != 0 {
                        uu = up[k][uu];
                    }
                }
                if uu != vv {
                    for k in (0..=LOG).rev() {
                        if up[k][uu] != up[k][vv] {
                            uu = up[k][uu];
                            vv = up[k][vv];
                        }
                    }
                    uu = up[0][uu];
                }
                let l = uu;

                let mut xu = 0u32;
                let mut i = ti[u];
                while i > 0 {
                    xu ^= bit[i];
                    i -= i & (!i + 1);
                }
                let mut xv = 0u32;
                let mut i = ti[v];
                while i > 0 {
                    xv ^= bit[i];
                    i -= i & (!i + 1);
                }
                let x = xu ^ xv ^ node_bit[l];
                res.push(x == 0 || (x & (x - 1)) == 0);
            }
        }
        res
    }
}