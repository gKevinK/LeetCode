impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut bucket = vec![vec![]; 82];
        for i in 0..n {
            let mut num = nums[i];
            let mut sum = 0;
            while num > 0 {
                sum += num % 10;
                num /= 10;
            }
            bucket[sum as usize].push((nums[i], i));
        }
        let mut v = vec![];
        for i in 0..82 {
            bucket[i].sort();
            v.extend(bucket[i].iter().map(|x| x.1));
        }
        let mut res = 0;
        for i in 0..n {
            if v[i] == n {
                continue;
            }
            let mut j = v[i];
            v[i] = n;
            while j != i {
                res += 1;
                let nj = v[j];
                v[j] = n;
                j = nj;
            }
        }
        res
    }
}