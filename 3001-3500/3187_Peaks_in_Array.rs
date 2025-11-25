impl Solution {

    fn is_peak(nums: &Vec<i32>, index: i32) -> bool {
        if index <= 0 || index as usize + 1 >= nums.len() {
            return false;
        }
        let index = index as usize;
        nums[index - 1] < nums[index] && nums[index] > nums[index + 1]
    }

    fn st_add(mut t: &mut Vec<i32>, mut i: usize, add: i32) {
        i += 1;
        while i < t.len() {
            t[i] += add;
            i += i & i.wrapping_neg();
        }
    }

    fn st_get(t: &Vec<i32>, mut i: usize) -> i32 {
        let mut res = 0;
        while i > 0 {
            res += t[i];
            i -= i & i.wrapping_neg();
        }
        res
    }

    pub fn count_of_peaks(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut st = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            if Self::is_peak(&nums, i as i32) {
                Self::st_add(&mut st, i, 1);
            }
        }

        let mut res = vec![];
        for query in queries {
            if query[0] == 1 {
                if query[1] + 1 >= query[2] {
                    res.push(0);
                } else {
                    res.push(Self::st_get(&st, query[2] as usize) - Self::st_get(&st, query[1] as usize + 1));
                }
            } else {
                let mut peak_c = Self::is_peak(&nums, query[1]) as i32;
                let mut peak_l = Self::is_peak(&nums, query[1] - 1) as i32;
                let mut peak_r = Self::is_peak(&nums, query[1] + 1) as i32;
                nums[query[1] as usize] = query[2];
                peak_c = Self::is_peak(&nums, query[1]) as i32 - peak_c;
                peak_l = Self::is_peak(&nums, query[1] - 1) as i32 - peak_l;
                peak_r = Self::is_peak(&nums, query[1] + 1) as i32 - peak_r;
                if peak_c != 0 {
                    Self::st_add(&mut st, query[1] as usize, peak_c);
                }
                if peak_l != 0 {
                    Self::st_add(&mut st, query[1] as usize - 1, peak_l);
                }
                if peak_r != 0 {
                    Self::st_add(&mut st, query[1] as usize + 1, peak_r);
                }
            }
        }
        res
    }
}