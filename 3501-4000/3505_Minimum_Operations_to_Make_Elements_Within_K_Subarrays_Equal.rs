use std::collections::BTreeMap;
use std::collections::btree_map::{Entry, OccupiedEntry};

impl Solution {
    fn rm_one(m: &mut BTreeMap<i32, i32>, k: i32) {
        if let Entry::Occupied(mut e) = m.entry(k) {
            *e.get_mut() -= 1;
            if *e.get() <= 0 {
                e.remove();
            }
        }
    }

    pub fn min_operations(nums: Vec<i32>, x: i32, k: i32) -> i64 {
        let ux = x as usize;
        let mut ops = vec![0i64; nums.len() - x as usize + 1];

        let mut set1 = BTreeMap::new();
        let mut set2 = BTreeMap::new();
        let (mut sum1, mut count1) = (0i64, 0i64);
        let (mut sum2, mut count2) = (0i64, 0i64);
        for i in 0..nums.len() {
            if i >= ux {
                let ix = i - ux;
                if set1.contains_key(&nums[ix]) {
                    Self::rm_one(&mut set1, nums[ix]);
                    sum1 -= nums[ix] as i64;
                    count1 -= 1;
                } else {
                    Self::rm_one(&mut set2, nums[ix]);
                    sum2 -= nums[ix] as i64;
                    count2 -= 1;
                }
            }
            if count1 < count2 {
                *set1.entry(nums[i]).or_insert(0) += 1;
                sum1 += nums[i] as i64;
                count1 += 1;
            } else {
                *set2.entry(nums[i]).or_insert(0) += 1;
                sum2 += nums[i] as i64;
                count2 += 1;
            }
            while count1 > 0 && count2 > 0 {
                let mut s1_max_entry = set1.last_entry().unwrap();
                let mut s2_min_entry = set2.first_entry().unwrap();
                let s1_max = *s1_max_entry.key();
                let s2_min = *s2_min_entry.key();
                if s1_max > s2_min {
                    sum1 -= s1_max as i64;
                    sum2 += s1_max as i64;
                    sum1 += s2_min as i64;
                    sum2 -= s2_min as i64;
                    *s1_max_entry.get_mut() -= 1;
                    if *s1_max_entry.get() == 0 {
                        s1_max_entry.remove_entry();
                    }
                    *s2_min_entry.get_mut() -= 1;
                    if *s2_min_entry.get() == 0 {
                        s2_min_entry.remove_entry();
                    }
                    *set1.entry(s2_min).or_insert(0) += 1;
                    *set2.entry(s1_max).or_insert(0) += 1;
                } else {
                    break;
                }
            }
            if ux - 1 <= i {
                let median = *set2.first_key_value().unwrap().0 as i64;
                ops[i + 1 - ux] = count1 * median - sum1
                                + sum2 - count2 * median;
            }
        }

        let mut dp1 = vec![0i64; nums.len() + 1];
        let mut dp2 = vec![0i64; nums.len() + 1];
        for ik in 0..k as usize {
            (dp1, dp2) = (dp2, dp1);
            dp2[ux * (ik + 1) - 1] = 1_000_000_000_000i64;
            for i in (ux * ik)..=(nums.len() - (k as usize - ik) * ux) {
                dp2[i + ux] = std::cmp::min(dp2[i + ux - 1], dp1[i] + ops[i]);
            }
        }

        *dp2.last().unwrap()
    }
}