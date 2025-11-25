impl Solution {
    pub fn sum_imbalance_numbers(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut res = 0;
        for i in 0..n {
            let mut visit = [false; 1002];
            visit[nums[i] as usize] = true;
            let mut count = 0;
            for j in (i + 1)..n {
                let nj = nums[j] as usize;
                if !visit[nj] {
                    if visit[nj - 1] && visit[nj + 1] {
                        count -= 1;
                    } else if !visit[nj - 1] && !visit[nj + 1] {
                        count += 1;
                    }
                }
                visit[nj] = true;
                res += count;
            }
        }
        res
    }
}