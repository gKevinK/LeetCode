impl Solution {
    pub fn popcount_depth(mut nums: Vec<i64>, queries: Vec<Vec<i64>>) -> Vec<i32> {
        let n = nums.len();
        let mut pd = vec![0; n];

        let depth = |mut num: i64| {
            let mut d = 0;
            while num > 1 {
                num = num.count_ones() as _;
                d += 1;
            }
            d
        };
        let mut trees = vec![vec![0; n + 1]; 6];
        for i in 0..n {
            pd[i] = depth(nums[i]);
            Self::add(&mut trees[pd[i]], i, 1);
        }

        let mut res = vec![];
        for query in &queries {
            if query[0] == 1 {
                let l = query[1] as usize;
                let r = query[2] as usize;
                let lc = Self::get(&trees[query[3] as usize], l);
                let rc = Self::get(&trees[query[3] as usize], r + 1);
                res.push((rc - lc) as i32);
            } else {
                let idx = query[1] as usize;
                nums[idx] = query[2];
                let mut pd_new = depth(query[2]);
                if pd[idx] != pd_new {
                    Self::add(&mut trees[pd[idx]], idx, -1);
                    Self::add(&mut trees[pd_new], idx, 1);
                    pd[idx] = pd_new;
                }
            }
        }
        res
    }

    fn add(mut t: &mut Vec<i64>, mut i: usize, add: i64) {
        i += 1;
        while i < t.len() {
            t[i] += add;
            i += i & i.wrapping_neg();
        }
    }

    fn get(t: &Vec<i64>, mut i: usize) -> i64 {
        let mut res = 0;
        while i > 0 {
            res += t[i];
            i -= i & i.wrapping_neg();
        }
        res
    }
}