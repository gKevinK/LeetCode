impl Solution {
    pub fn min_reorder(n: i32, mut connections: Vec<Vec<i32>>) -> i32 {
        let mut c2e = vec![vec![]; n as usize];
        for i in 0..connections.len() {
            c2e[connections[i][0] as usize].push(i);
            c2e[connections[i][1] as usize].push(i);
        }
        let mut r = 0;
        let mut q = std::collections::VecDeque::new();
        for i in &c2e[0] {
            if connections[*i][0] == 0 {
                r += 1;
                connections[*i].swap(0, 1);
            }
            q.push_back(*i);
        }
        while let Some(i) = q.pop_front() {
            for e in &c2e[connections[i][0] as usize] {
                if *e == i {
                    continue;
                }
                if connections[*e][0] == connections[i][0] {
                    r += 1;
                    connections[*e].swap(0, 1);
                }
                q.push_back(*e);
            }
        }
        r
    }
}