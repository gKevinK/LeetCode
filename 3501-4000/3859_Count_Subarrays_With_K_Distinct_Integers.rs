impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32, m: i32) -> i64 {
        let n = nums.len();
        if k * m > n as i32 {
            return 0;
        }
        let mut max_val = *nums.iter().max().unwrap_or(&0) as usize;
        let (mut left_k, mut left_k1, mut left_m) = (0, 0, 0);
        let (mut n_k, mut n_k1) = (0, 0);
        let mut b_m = 0;
        let mut count_k = vec![0; max_val + 1];
        let mut count_k1 = vec![0; max_val + 1];
        let mut count_m = vec![0; max_val + 1];
        let mut res = 0;
        for r in 0..n {
            let v = nums[r] as usize;

            if count_k[v] == 0 {
                n_k += 1;
            }
            count_k[v] += 1;
            while n_k > k {
                count_k[nums[left_k] as usize] -= 1;
                if count_k[nums[left_k] as usize] == 0 {
                    n_k -= 1;
                }
                left_k += 1;
            }

            if count_k1[v] == 0 {
                n_k1 += 1;
            }
            count_k1[v] += 1;
            while n_k1 > k - 1 {
                count_k1[nums[left_k1] as usize] -= 1;
                if count_k1[nums[left_k1] as usize] == 0 {
                    n_k1 -= 1;
                }
                left_k1 += 1;
            }

            if count_m[v] == 0 && m > 1 {
                b_m += 1;
            } else if count_m[v] + 1 == m && m > 1 {
                b_m -= 1;
            }
            count_m[v] += 1;
            while left_m < left_k1 {
                let x = nums[left_m] as usize;
                if left_m < left_k || count_m[x] > m {
                    if count_m[x] == m && m > 1 {
                        b_m += 1;
                    } else if count_m[x] == 1 && m > 1 {
                        b_m -= 1;
                    }
                    count_m[x] -= 1;
                    left_m += 1;
                } else {
                    break;
                }
            }

            let right = left_k1.min(left_m + 1);
            if n_k == k && b_m == 0 {
                res += (right - left_k) as i64;
            }
        }
        res
    }
}