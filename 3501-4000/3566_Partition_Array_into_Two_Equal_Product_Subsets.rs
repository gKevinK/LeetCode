impl Solution {
    const PRIME: [i32; 25] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];

    pub fn check_equal_partitions(nums: Vec<i32>, target: i64) -> bool {
        let plen = Self::PRIME.len();
        let mut f = vec![0; plen];
        let mut t = target;
        for j in 0..plen {
            let mut count = 0;
            let p = Self::PRIME[j] as i64;
            while t % p == 0 {
                count += 1;
                t /= p;
            }
            f[j] = count;
        }
        if t > 1 {
            return false;
        }

        let n = nums.len();
        let mut fnums = vec![vec![0; plen]; nums.len()];
        for i in 0..n {
            let mut num = nums[i];
            for j in 0..plen {
                let mut count = 0;
                let p = Self::PRIME[j];
                while num % p == 0 {
                    count += 1;
                    num /= p;
                }
                fnums[i][j] = count;
            }
        }
        for j in 0..plen {
            if (0..n).map(|i| fnums[i][j]).sum::<i32>() != f[j] * 2 {
                return false;
            }
        }

        let mut q = std::collections::VecDeque::from([(0, 1), (0, 0)]);
        let mut curr = vec![0; plen];
        while let Some((i, o)) = q.pop_front() {
            if o == 1 {
                for j in 0..plen {
                    curr[j] += fnums[i][j];
                }
            } else {
                for j in 0..plen {
                    curr[j] -= fnums[i][j];
                }
            }
            if (0..plen).all(|j| curr[j] <= f[j]) {
                if i + 1 == n {
                    if curr == f {
                        return true;
                    }
                } else {
                    q.push_front((i + 1, 0));
                    q.push_front((i + 1, 1));
                }
            }
        }
        false
    }
}