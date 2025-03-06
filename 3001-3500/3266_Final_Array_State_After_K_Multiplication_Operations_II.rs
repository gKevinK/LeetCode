impl Solution {
    fn power_mod(mut base: i64, mut exp: i64, m: i64) -> i64 {
        let mut r = 1;
        while exp > 0 {
            if exp % 2 == 1 {
                r = (r * base) % m;
            }
            base = (base * base) % m;
            exp /= 2;
        }
        r
    }

    pub fn get_final_state(mut nums: Vec<i32>, mut k: i32, multiplier: i32) -> Vec<i32> {
        use std::collections::{BinaryHeap, HashMap};
        use std::cmp::Reverse;
        if multiplier == 1 {
            return nums;
        }
        const MOD: i64 = 1_000_000_007;
        let mut heap = BinaryHeap::new();
        for i in 0..nums.len() {
            heap.push((Reverse(nums[i] as i64), Reverse(i)));
        }
        let mut m = HashMap::new();
        while k > 0 {
            if m.len() == nums.len() {
                break;
            }
            k -= 1;
            let (Reverse(n), Reverse(i)) = heap.pop().unwrap();
            heap.push((Reverse(n * multiplier as i64), Reverse(i)));
            *m.entry(i).or_insert(0) += 1;
        }
        let round = k as i64 / nums.len() as i64;
        let mut rest = k as i64 % nums.len() as i64;
        while let Some((Reverse(n), Reverse(i))) = heap.pop() {
            nums[i] = (((n % MOD)
                * Self::power_mod(multiplier as i64,
                    round + if rest > 0 { 1 } else { 0 }, MOD)
                ) % MOD) as i32;
            rest -= 1;
        }
        nums
    }
}