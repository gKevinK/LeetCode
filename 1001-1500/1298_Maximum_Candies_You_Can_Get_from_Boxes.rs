impl Solution {
    pub fn max_candies(status: Vec<i32>, candies: Vec<i32>, keys: Vec<Vec<i32>>, contained_boxes: Vec<Vec<i32>>, initial_boxes: Vec<i32>) -> i32 {
        let n = status.len();
        let mut v = status;
        let mut q = std::collections::VecDeque::new();
        for i in initial_boxes {
            v[i as usize] |= 0x10;
            q.push_back(i);
        }
        let mut r = 0;
        while let Some(i) = q.pop_front() {
            if v[i as usize] != 0x11 {
                continue;
            }
            v[i as usize] = 0;
            r += candies[i as usize];
            for &k in &keys[i as usize] {
                v[k as usize] |= 0x01;
                q.push_back(k);
            }
            for &b in &contained_boxes[i as usize] {
                v[b as usize] |= 0x10;
                q.push_back(b);
            }
        }
        r
    }
}