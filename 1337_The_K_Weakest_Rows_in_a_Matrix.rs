impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        use std::collections::BinaryHeap;
        let mut heap = BinaryHeap::new();
        for i in 0..mat.len() {
            let mut soldiers = 0;
            for j in 0..mat[i].len() {
                if mat[i][j] == 1 {
                    soldiers += 1;
                } else {
                    break;
                }
            }
            heap.push((soldiers, i as i32));
            if heap.len() > k as usize {
                heap.pop();
            }
        }
        let mut res = Vec::new();
        while let Some(p) = heap.pop() {
            res.push(p.1);
        }
        res.reverse();
        res
    }
}