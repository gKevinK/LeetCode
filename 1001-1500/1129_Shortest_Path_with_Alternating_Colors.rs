impl Solution {
    pub fn shortest_alternating_paths(n: i32, red_edges: Vec<Vec<i32>>, blue_edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res_red = vec![-1; n as usize];
        let mut res_blue = vec![-1; n as usize];
        res_red[0] = 0;
        res_blue[0] = 0;
        let mut map_red = vec![Vec::new(); n as usize];
        let mut map_blue = vec![Vec::new(); n as usize];
        for e in &red_edges {
            map_red[e[0] as usize].push(e[1]);
        }
        for e in &blue_edges {
            map_blue[e[0] as usize].push(e[1]);
        }
        let mut deque = std::collections::VecDeque::new();
        deque.push_back((0, 0, 0));
        deque.push_back((0, 1, 0));
        while let Some(r) = deque.pop_front() {
            if r.1 == 0 {
                for _n in &map_blue[r.0 as usize] {
                    let next = *_n;
                    if res_blue[next as usize] != -1 {
                        continue;
                    }
                    res_blue[next as usize] = r.2 + 1;
                    deque.push_back((next, 1 - r.1, r.2 + 1));
                }
            } else {
                for _n in &map_red[r.0 as usize] {
                    let next = *_n;
                    if res_red[next as usize] != -1 {
                        continue;
                    }
                    res_red[next as usize] = r.2 + 1;
                    deque.push_back((next, 1 - r.1, r.2 + 1));
                }
            }
        }
        let mut res = Vec::new();
        for i in 0..n as usize {
            res.push(if res_red[i] >= 0 && res_blue[i] >= 0 {
                std::cmp::min(res_red[i], res_blue[i])
            } else {
                std::cmp::max(res_red[i], res_blue[i])
            });
        }
        res
    }
}