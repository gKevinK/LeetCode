impl Solution {
    pub fn prime_sub_operation(mut nums: Vec<i32>) -> bool {
        let n_max = *nums.iter().max().unwrap() as usize;
        let mut is_prime = vec![true; n_max as usize + 1];
        is_prime[0] = false;
        is_prime[1] = false;
        for i in 2..n_max {
            if is_prime[i] {
                let mut j = 2;
                while i * j < n_max {
                    is_prime[i * j] = false;
                    j += 1;
                }
            }
        }
        let mut primes = vec![];
        for i in 2..n_max {
            if is_prime[i] {
                primes.push(i as i32);
            }
        }
        for i in (0..(nums.len() - 1)).rev() {
            if nums[i] >= nums[i + 1] {
                for j in 0..primes.len() {
                    if primes[j] >= nums[i] {
                        break;
                    }
                    if nums[i] - primes[j] < nums[i + 1] {
                        nums[i] -= primes[j];
                        break;
                    }
                }
            }
            if nums[i] >= nums[i + 1] {
                return false;
            }
        }
        true
    }
}