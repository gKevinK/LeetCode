impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut count = [0; 201];
        for n in nums {
            count[n as usize + 100] += 1;
        }
        let mut res = 0;
        let mut flag = false;
        for i in (0..=200).rev() {
            if count[i] > 0 {
                res += i as i32 - 100;
                flag = true;
            }
            if flag && i <= 100 {
                break;
            }
        }
        res
    }
}