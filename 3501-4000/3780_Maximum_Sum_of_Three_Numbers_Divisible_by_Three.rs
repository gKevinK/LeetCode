impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut x = [vec![], vec![], vec![]];
        for &num in &nums {
            let r = (num % 3) as usize;
            x[r].push(num);
            x[r].sort_by_key(|&n| -n);
            if x[r].len() == 4 {
                x[r].pop();
            }
        }
        let mut res = 0;
        for r in 0..3 {
            if x[r].len() == 3 {
                res = res.max(x[r][0] + x[r][1] + x[r][2]);
            }
        }
        if x[0].len() > 0 && x[1].len() > 0 && x[2].len() > 0 {
            res = res.max(x[0][0] + x[1][0] + x[2][0]);
        }
        res
    }
}