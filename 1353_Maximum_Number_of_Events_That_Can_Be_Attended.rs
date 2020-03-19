impl Solution {
    pub fn max_events(mut events: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;

        events.sort_by_key(|e| e[0]);
        let mut r = 0;
        let mut h = std::collections::BinaryHeap::new();
        let mut t = 0;
        let mut i = 0;
        while i < events.len() || !h.is_empty() {
            while i < events.len() && events[i][0] <= t {
                h.push(Reverse(events[i][1]));
                i += 1;
            }
            while let Some(Reverse(end)) = h.pop() {
                if end >= t {
                    r += 1;
                    break;
                }
            }
            t += 1;
        }
        r
    }
}