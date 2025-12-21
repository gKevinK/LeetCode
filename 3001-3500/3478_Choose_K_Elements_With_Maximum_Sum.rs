impl Solution {
    pub fn find_max_sum(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i64> {
        let n = nums1.len();
        let mut v = vec![];
        for i in 0..n {
            v.push((nums1[i], i));
        }
        v.sort_unstable();
        let mut heap = std::collections::BinaryHeap::new();
        let mut sum = 0i64;
        let mut res = vec![0i64; n];
        let mut last_n = v[0].0;
        let mut wait = vec![];
        for (num, i) in v {
            if num != last_n {
                for w in wait {
                    heap.push(std::cmp::Reverse(w));
                    sum += w as i64;
                    if heap.len() > k as usize {
                        sum -= heap.pop().unwrap().0 as i64;
                    }
                }
                last_n = num;
                wait = vec![];
            }
            res[i] = sum;
            wait.push(nums2[i]);
        }
        res
    }
}