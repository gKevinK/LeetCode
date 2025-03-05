use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    const pow10: [i32; 7] = [1, 10, 100, 1_000, 10_000, 100_000, 1_000_000];

    pub fn count_pairs(mut nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut freq = HashMap::new();
        nums.sort_unstable();
        for num in nums {
            let mut s = HashSet::new();
            let mut len = 0;
            let mut numx = num;
            while numx > 0 {
                len += 1;
                numx /= 10;
            }
            s.insert(num);
            for i in 0..len {
                for j in (i + 1)..len {
                    let di = (num / Self::pow10[i]) % 10;
                    let dj = (num / Self::pow10[j]) % 10;
                    let num1 = num
                        - di * Self::pow10[i] + di * Self::pow10[j]
                        - dj * Self::pow10[j] + dj * Self::pow10[i];
                    s.insert(num1);
                    for k in 0..len {
                        for l in (k + 1)..len {
                            let dk = (num1 / Self::pow10[k]) % 10;
                            let dl = (num1 / Self::pow10[l]) % 10;
                            let num2 = num1
                                - dk * Self::pow10[k] + dk * Self::pow10[l]
                                - dl * Self::pow10[l] + dl * Self::pow10[k];
                            s.insert(num2);
                        }
                    }
                }
            }
            for v in s {
                res += freq.get(&v).unwrap_or(&0);
            }
            *freq.entry(num).or_insert(0) += 1;
        }
        res
    }
}