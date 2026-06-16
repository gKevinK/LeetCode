struct Time(f64);

impl PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Time {}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for Time {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.partial_cmp(&self.0).unwrap()
    }
}

impl Solution {
    pub fn min_time(n: i32, k: i32, m: i32, time: Vec<i32>, mul: Vec<f64>) -> f64 {
        let n = n as usize;
        let m = m as usize;
        let all = (1 << n) - 1;
        let MAX = 100000.0;
        let mut heap = std::collections::BinaryHeap::from([(Time(0.0), 0, 0, 0)]);
        let mut seen = vec![MAX; (all + 1) * 2 * m as usize];
        let idx = |mask: usize, boat: usize, stage: usize| (mask * 2 + boat) * m + stage;
        seen[idx(0, 0, 0)] = 0.0;
        while let Some((Time(t), mask, boat, stage)) = heap.pop() {
            if mask == all && boat == 1 {
                return (t * 100000.0).round() / 100000.0;
            }
            let x = idx(mask, boat, stage);
            if t > seen[x] {
                continue;
            }
            seen[x] = t;
            if boat == 1 {
                for i in 0..n {
                    if (1 << i) & mask > 0 {
                        let need_time = time[i] as f64 * mul[stage];
                        let new_time = t + need_time;
                        let new_mask = mask - (1 << i);
                        let new_stage = (stage + need_time as usize) % m;
                        let x2 = idx(new_mask, 0, new_stage);
                        if new_time < seen[x2] {
                            seen[x2] = new_time;
                            heap.push((Time(new_time), new_mask, 0, new_stage));
                        }
                    }
                }
            } else {
                let rmask = (!mask) & all;
                let mut this = all & rmask;
                while this > 0 {
                    if this.count_ones() as i32 <= k {
                        let need_time = mul[stage] * (0..n)
                            .filter(|i| (1 << i) & this > 0)
                            .map(|i| time[i]).max().unwrap() as f64;
                        let new_time = t + need_time;
                        let new_mask = mask | this;
                        let new_stage = (stage + need_time as usize) % m;
                        let x2 = idx(new_mask, 1, new_stage);
                        if new_time < seen[x2] {
                            seen[x2] = new_time;
                            heap.push((Time(new_time), new_mask, 1, new_stage));
                        }
                    }
                    this = (this - 1) & rmask;
                }
            }
        }
        -1.0
    }
}