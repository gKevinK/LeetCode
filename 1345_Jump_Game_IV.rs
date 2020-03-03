impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let mut v = vec![std::i32::MAX; arr.len()];
        let mut m = std::collections::HashMap::new();
        for i in arr.iter().enumerate() {
            if i.0 > 0 && i.0 + 1 < arr.len() && arr[i.0 - 1] == *i.1 && arr[i.0 + 1] == *i.1 {
                continue;
            }
            m.entry(i.1).or_insert_with(Vec::new).push(i.0);
        }
        let mut q = std::collections::VecDeque::new();
        q.push_back((0, 0));
        while let Some(p) = q.pop_front() {
            if v[p.1] != std::i32::MAX {
                continue;
            }
            v[p.1] = p.0;
            if p.1 + 1 == arr.len() {
                return p.0;
            }
            if p.1 + 1 < arr.len() {
                q.push_back((p.0 + 1, p.1 + 1));
            }
            if p.1 > 0 {
                q.push_back((p.0 + 1, p.1 - 1));
            }
            for x in m.get(&arr[p.1]).unwrap() {
                q.push_back((p.0 + 1, *x));
            }
        }
        *v.last().unwrap()
    }
}