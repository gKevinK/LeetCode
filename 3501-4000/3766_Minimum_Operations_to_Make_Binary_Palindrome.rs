impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> Vec<i32> {
        use std::sync::LazyLock;
        static pal: LazyLock<Vec<i32>> = LazyLock::new(|| {
            let mut p = vec![];
            for i in 1..5500 {
                let mut x = i;
                let mut y = 0;
                while x > 0 {
                    y = (y << 1) + (x & 1);
                    x >>= 1;
                }
                if i == y {
                    p.push(i);
                }
            }
            p
        });

        let mut res = vec![0; nums.len()];
        for i in 0..nums.len() {
            let num = nums[i];
            let x = pal.partition_point(|&p| p < num);
            let mut min = pal[x] - num;
            if x > 0 {
                min = min.min(num - pal[x - 1]);
            }
            res[i] = min;
        }
        res
    }
}