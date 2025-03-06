impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        if n as usize > connections.len() + 1 {
            return -1;
        }
        let mut parent: Vec<_> = (0..n as usize).collect();
        let mut depth = vec![0; n as usize];
        for c in &connections {
            let mut a = c[0] as usize;
            let mut b = c[1] as usize;
            while parent[a] != a {
                a = parent[a];
            }
            while parent[b] != b {
                b = parent[b];
            }
            if a != b {
                if depth[a] < depth[b] {
                    let temp = b;
                    b = a;
                    a = temp;
                }
                parent[b] = a;
                depth[a] = std::cmp::max(depth[a], depth[b] + 1);
            }
        }
        let mut r = 0;
        for i in 0..(n as usize) {
            if parent[i] == i {
                r += 1;
            }
        }
        r - 1
    }
}