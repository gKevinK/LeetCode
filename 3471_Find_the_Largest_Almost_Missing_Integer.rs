impl Solution {
    pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        if k == 1 || n == k as usize {
            let mut c = vec![0; 51];
            for i in 0..n {
                c[nums[i] as usize] += 1;
            }
            if k == 1 {
                for x in (0..=50).rev() {
                    if c[x] == 1 {
                        return x as i32;
                    }
                }
                return -1;
            } else {
                for x in (0..=50).rev() {
                    if c[x] > 0 {
                        return x as i32;
                    }
                }
            }
        }
        let x1 = nums[0];
        let mut c1 = 0;
        let x2 = nums[n - 1];
        let mut c2 = 0;
        for i in 0..n {
            if nums[i] == x1 {
                c1 += 1;
            }
            if nums[i] == x2 {
                c2 += 1;
            }
            if c1 > 1 && c2 > 1 {
                break;
            }
        }
        if c1 == 1 && c2 == 1 {
            std::cmp::max(x1, x2)
        } else if c1 == 1 {
            x1
        } else if c2 == 1 {
            x2
        } else {
            -1
        }
    }
}