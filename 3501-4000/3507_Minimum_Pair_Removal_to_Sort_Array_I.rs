impl Solution {
    pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
        let mut n1 = nums.clone();
        let mut n2 = Vec::with_capacity(nums.len());
        let mut ops = 0;
        while !n1.windows(2).all(|w| w[0] <= w[1]) {
            let mut sum = i32::MAX;
            let mut idx = 0;
            for i in 0..(n1.len() - 1) {
                if n1[i] + n1[i + 1] < sum {
                    idx = i;
                    sum = n1[i] + n1[i + 1];
                }
            }
            for i in 0..idx {
                n2.push(n1[i]);
            }
            n2.push(sum);
            for i in (idx + 2)..n1.len() {
                n2.push(n1[i]);
            }
            n1.clear();
            (n1, n2) = (n2, n1);
            ops += 1;
        }
        ops
    }
}