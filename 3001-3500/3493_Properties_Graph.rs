impl Solution {
    pub fn number_of_components(properties: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut pf = Vec::new();
        for v in properties {
            let mut vf = vec![0; 101];
            for p in v {
                vf[p as usize] = 1;
            }
            pf.push(vf);
        }

        let n = pf.len();
        let mut graph = vec![vec![]; n];
        for i in 1..n {
            for j in 0..i {
                let mut count = 0;
                for k in 1..=100 {
                    if pf[i][k] == 1 && pf[j][k] == 1 {
                        count += 1;
                    }
                }
                if count >= k {
                    graph[i].push(j);
                    graph[j].push(i);
                }
            }
        }

        println!("{:?}", graph);

        let mut comps = 0;
        let mut visit = vec![false; n];
        let mut q = std::collections::VecDeque::new();
        for i in 0..n {
            if visit[i] {
                continue;
            }
            visit[i] = true;
            comps += 1;
            q.clear();
            q.push_back(i);
            while let Some(x) = q.pop_front() {
                for &y in &graph[x] {
                    if visit[y] == false {
                        visit[y] = true;
                        q.push_back(y);
                    }
                }
                visit[x] = true;
            }
        }
        comps
    }
}