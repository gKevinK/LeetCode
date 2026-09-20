const LIMIT: usize = 100_001;

const KERNEL: [i32; LIMIT] = {
    let mut k = [0; LIMIT];
    let mut i = 1;
    while i < LIMIT {
        if k[i] != 0 {
            i += 1;
            continue;
        }
        let mut j = 1;
        while i * j * j < LIMIT {
            k[i * j * j] = i as i32;
            j += 1;
        }
        i += 1;
    }
    k
};

impl Solution {
    pub fn sum_of_ancestors(n: i32, edges: Vec<Vec<i32>>, nums: Vec<i32>) -> i64 {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for e in &edges {
            g[e[0] as usize].push(e[1] as usize);
            g[e[1] as usize].push(e[0] as usize);
        }
        let mut k = nums.iter().map(|&num| KERNEL[num as usize]).collect::<Vec<_>>();

        let mut visit = vec![false; n];
        let mut count = std::collections::HashMap::new();
        let mut queue = std::collections::VecDeque::from([(0, 0), (1, 0)]);
        let mut res = 0;
        while let Some((io, node)) = queue.pop_front() {
            let e = count.entry(k[node]).or_insert(0);
            if io == 0 {
                res += *e;
                *e += 1;
                visit[node] = true;
                for &next in &g[node] {
                    if !visit[next] {
                        queue.push_front((1, next));
                        queue.push_front((0, next));
                    }
                }
            } else {
                *e -= 1;
            }
        }
        res
    }
}