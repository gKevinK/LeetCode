impl Solution {
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Solution::gcd(b, a % b)
        }
    }

    pub fn min_operations(nums: Vec<i32>, nums_divide: Vec<i32>) -> i32 {
        let mut cd = nums_divide[0];
        for &n in nums_divide.iter().skip(1) {
            cd = Solution::gcd(cd, n);
        }
        let mut cd2 = cd;
        let mut flag = false;
        for &n in &nums {
            if Solution::gcd(n, cd) == n {
                cd2 = std::cmp::min(cd2, n);
                flag = true;
            }
        }
        if !flag {
            return -1;
        }
        let mut del = 0;
        for &n in &nums {
            if n < cd2 {
                del += 1;
            }
        }
        del
    }
}