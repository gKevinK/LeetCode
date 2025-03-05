impl Solution {
    pub fn minimum_operations(nums: Vec<i32>, start: i32, goal: i32) -> i32 {
        let mut visit = vec![false; 1001];
        visit[start as usize] = true;
        let mut s1 = std::collections::HashSet::new();
        let mut step = 0;
        s1.insert(start);
        while !s1.is_empty() {
            step += 1;
            let mut s2 = std::collections::HashSet::new();
            for x in &s1 {
                for n in &nums {
                    for &t in [x + n, x - n, x ^ n].iter() {
                        if t == goal {
                            return step;
                        }
                        if t < 0 || 1000 < t {
                            continue;
                        }
                        if !s2.contains(&t) && !visit[t as usize] {
                            visit[t as usize] = true;
                            s2.insert(t);
                        }
                    }
                }
            }
            s1 = s2;
        }
        -1
    }
}