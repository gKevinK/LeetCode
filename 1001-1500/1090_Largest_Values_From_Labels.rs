impl Solution {
    pub fn largest_vals_from_labels(values: Vec<i32>, labels: Vec<i32>, num_wanted: i32, use_limit: i32) -> i32 {
        use std::collections::HashMap;
        use std::collections::BinaryHeap;

        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        for i in 0..values.len() {
            map.entry(labels[i]).or_insert(Vec::new()).push(values[i]);
        }
        let mut heap: BinaryHeap<(i32, i32, i32)> = BinaryHeap::new();
        for (k, v) in &mut map {
            v.sort();
            heap.push((*v.last().unwrap(), *k, 0));
        }
        let mut res = 0;
        let mut num = 0;
        while !heap.is_empty() && num < num_wanted {
            let t = heap.pop().unwrap();
            res += t.0;
            map.get_mut(&t.1).unwrap().pop();
            if t.2 + 1 < use_limit && !map[&t.1].is_empty() {
                heap.push((*map[&t.1].last().unwrap(), t.1, t.2 + 1));
            }
            num += 1;
        }
        res
    }
}