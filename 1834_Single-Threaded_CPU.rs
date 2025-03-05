impl Solution {
    pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        use std::cmp::Reverse;
        let mut heap = std::collections::BinaryHeap::new();
        let mut order = Vec::new();
        let mut task_i = 0;
        let mut time = 0;
        let mut ts = Vec::new();
        for i in 0..tasks.len() {
            ts.push((tasks[i][0], tasks[i][1], i as i32));
        }
        ts.sort();
        while !heap.is_empty() || task_i != ts.len() {
            while task_i < ts.len() && ts[task_i].0 <= time {
                heap.push((Reverse(ts[task_i].1), Reverse(ts[task_i].2)));
                task_i += 1;
            }
            if let Some((Reverse(pt), Reverse(ti))) = heap.pop() {
                time += pt;
                order.push(ti);
            } else if task_i < ts.len() && ts[task_i].0 > time {
                time = ts[task_i].0;
            }
        }
        order
    }
}