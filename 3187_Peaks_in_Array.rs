impl Solution {

    fn is_peak(nums: &Vec<i32>, index: i32) -> bool {
        if index <= 0 || index as usize + 1 >= nums.len() {
            return false;
        }
        let index = index as usize;
        nums[index - 1] < nums[index] && nums[index] > nums[index + 1]
    }

    fn st_add(st: &mut Vec<i32>, mut index: usize, value: i32) {
        if value == 0 {
            return;
        }
        let mut mask = 1;
        while index < st.len() {
            if mask & index == 0 {
                st[index] += value;
                index |= mask;
            }
            mask <<= 1;
        }
    }

    fn st_query(st: &Vec<i32>, mut end: usize) -> i32 {
        let mut mask = 1;
        let mut res = 0;
        if end == 0 {
            return 0;
        }
        while mask <= end {
            if mask & end > 0 {
                res += st[end - 1];
                end -= mask;
            }
            mask <<= 1;
        }
        res
    }

    pub fn count_of_peaks(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut st = vec![0; nums.len()];
        for i in 0..nums.len() {
            if Self::is_peak(&nums, i as i32) {
                st[i] += 1;
            }
            let mut mask = 1;
            while mask <= i {
                if mask & i > 0 {
                    st[i] += st[i - mask];
                } else {
                    break;
                }
                mask <<= 1;
            }
        }
        let mut res = vec![];
        for query in queries {
            if query[0] == 1 {
                if query[1] + 1 >= query[2] {
                    res.push(0);
                } else {
                    res.push(Self::st_query(&st, query[2] as usize) - Self::st_query(&st, query[1] as usize + 1));
                }
            } else {
                let mut peak_c = Self::is_peak(&nums, query[1]) as i32;
                let mut peak_l = Self::is_peak(&nums, query[1] - 1) as i32;
                let mut peak_r = Self::is_peak(&nums, query[1] + 1) as i32;
                nums[query[1] as usize] = query[2];
                peak_c = Self::is_peak(&nums, query[1]) as i32 - peak_c;
                peak_l = Self::is_peak(&nums, query[1] - 1) as i32 - peak_l;
                peak_r = Self::is_peak(&nums, query[1] + 1) as i32 - peak_r;
                Self::st_add(&mut st, query[1] as usize, peak_c);
                Self::st_add(&mut st, query[1] as usize - 1, peak_l);
                Self::st_add(&mut st, query[1] as usize + 1, peak_r);
            }
        }
        res
    }
}