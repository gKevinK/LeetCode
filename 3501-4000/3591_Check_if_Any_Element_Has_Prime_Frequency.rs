impl Solution {
    const primes: [i32; 25] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];

    pub fn check_prime_frequency(nums: Vec<i32>) -> bool {
        let mut count = [0; 101];
        for num in nums {
            count[num as usize] += 1;
        }
        for c in count {
            if Self::primes.iter().find(|x| **x == c).is_some() {
                return true;
            }
        }
        false
    }
}