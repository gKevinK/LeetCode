impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;

        let n = nums1.len();
        let ku = k as usize;
        let mut v = (0..n).collect::<Vec<_>>();
        v.sort_unstable_by_key(|&i| -nums2[i]);
        let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        let mut sum = 0;
        let mut res = 0;
        for i in v {
            let n1 = nums1[i];
            if heap.len() < ku || heap.peek().unwrap().0 < n1 {
                heap.push(Reverse(n1));
                sum += n1 as i64;
            }
            if heap.len() > ku {
                sum -= heap.pop().unwrap().0 as i64;
            }
            if heap.len() == ku {
                res = res.max(sum * nums2[i] as i64);
            }
        }
        res
    }
}