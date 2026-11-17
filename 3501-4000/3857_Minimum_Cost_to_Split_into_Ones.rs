impl Solution {
    pub fn min_cost(n: i32) -> i32 {
        use std::sync::LazyLock;
        static result: LazyLock<Vec<i32>> = LazyLock::new(|| {
            let mut res = vec![0, 0, 1];
            while res.len() <= 500 {
                let mut m = i32::MAX;
                let num = res.len();
                for i in 1..=num / 2 {
                    m = m.min(res[i] + res[num - i] + i as i32 * (num - i) as i32);
                }
                res.push(m);
            }
            res
        });

        result[n as usize]
    }
}