impl Solution {
    pub fn result_array(nums: Vec<i32>, k: i32) -> Vec<i64> {
        let n = nums.len();
        let mut total = vec![0; k as usize];
        let mut curr = vec![0; k as usize];
        let mut next = vec![0; k as usize];
        for num in nums {
            let num = num % k;
            next.fill(0);
            for j in 0..k {
                let ju = j as usize;
                let new = ((j * num) % k) as usize;
                next[new] += curr[ju];
                total[new] += curr[ju];
            }
            next[num as usize] += 1;
            total[num as usize] += 1;
            std::mem::swap(&mut curr, &mut next);
        }
        total
    }
}