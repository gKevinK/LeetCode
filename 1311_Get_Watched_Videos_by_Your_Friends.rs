impl Solution {
    pub fn watched_videos_by_friends(watched_videos: Vec<Vec<String>>, friends: Vec<Vec<i32>>, id: i32, level: i32) -> Vec<String> {
        let n = watched_videos.len();
        let mut visit = vec![false; n];
        let mut q = std::collections::VecDeque::new();
        q.push_back(id);
        visit[id as usize] = true;
        for _ in 0..level {
            for _ in 0..q.len() {
                let i = q.pop_front().unwrap();
                for &i2 in &friends[i as usize] {
                    if !visit[i2 as usize] {
                        visit[i2 as usize] = true;
                        q.push_back(i2);
                    }
                }
            }
        }
        let mut m = std::collections::HashMap::new();
        while let Some(i) = q.pop_front() {
            for wv in &watched_videos[i as usize] {
                *m.entry(wv.clone()).or_insert(0) += 1;
            }
        }
        let mut m2 = std::collections::BTreeMap::new();
        for (k, v) in m {
            (*m2.entry(v).or_insert_with(Vec::new)).push(k);
        }
        let mut r = Vec::new();
        for (k, mut v) in m2 {
            v.sort();
            r.append(&mut v);
        }
        r
    }
}