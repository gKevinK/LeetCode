impl Solution {
    pub fn min_array_sum(mut nums: Vec<i32>, k: i32, mut op1: i32, mut op2: i32) -> i32 {
        use std::cmp::Reverse;

        let n = nums.len();
        nums.sort_by_key(|w| Reverse(*w));
        let mut i_2k = 0;
        let mut i_k = 0;
        for i in 0..nums.len() {
            if nums[i] >= k * 2 - 1 {
                i_2k = i + 1;
            }
            if nums[i] >= k {
                i_k = i + 1;
            }
        }

        for i in 0..i_2k {
            if op1 == 0 {
                break;
            }
            nums[i] = (nums[i] + 1) / 2;
            op1 -= 1;
        }
        for i in 0..i_2k {
            if op2 == 0 {
                break;
            }
            nums[i] -= k;
            op2 -= 1;
        }

        let mut s = std::collections::HashSet::new();
        let mut c = 0;
        let mut j = i_k;
        while op2 > 0 && i_2k < j {
            j -= 1;
            if k % 2 == 1 && nums[j] % 2 == 0 {
                s.insert(j);
            }
            nums[j] -= k;
            op2 -= 1;
        }
        for i in i_2k..j {
            if op1 == 0 {
                break;
            }
            if k % 2 == 1 && nums[i] % 2 == 1 {
                c += 1;
            }
            nums[i] = (nums[i] + 1) / 2;
            op1 -= 1;
        }

        let mut v = Vec::new();
        for i in j..n {
            v.push((nums[i], Reverse(i)));
        }
        v.sort();
        while let Some((num, Reverse(i))) = v.pop() {
            if op1 == 0 {
                break;
            }
            nums[i] = (nums[i] + 1) / 2;
            op1 -= 1;
            if c > 0 && s.contains(&i) {
                c -= 1;
                nums[i] -= 1;
            }
        }
        nums.iter().sum()
    }
}