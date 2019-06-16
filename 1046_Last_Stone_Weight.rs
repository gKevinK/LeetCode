impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        use std::collections::BinaryHeap;
        
        let mut heap = BinaryHeap::new();
        for i in stones {
            heap.push(i);
        }
        while heap.len() > 1 {
            let a = heap.pop().unwrap();
            let b = heap.pop().unwrap();
            if a != b {
                heap.push(a - b);
            }
        }
        heap.pop().unwrap_or(0)
    }
}