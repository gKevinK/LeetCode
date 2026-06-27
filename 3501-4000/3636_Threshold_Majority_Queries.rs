impl Solution {
    pub fn subarray_majority(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::HashMap;
        let n = nums.len();
        let mut inums = vec![0; n];
        let mut map = HashMap::new();
        let mut rmap = Vec::with_capacity(n / 4);
        for i in 0..n {
            if let Some(idx) = map.get(&nums[i]) {
                inums[i] = *idx;
            } else {
                inums[i] = rmap.len();
                rmap.push(nums[i]);
                map.insert(nums[i], inums[i]);
            }
        }
        let ndist = rmap.len();

        let bs = 1 + n.isqrt() / 2;
        let mut qs = queries.iter().enumerate()
                .map(|(i, q)| (q[0] as usize, q[1] as usize + 1, q[2], i)).collect::<Vec<_>>();
        qs.sort_unstable_by_key(|q| (q.0 / bs, q.1, n - q.0));

        let mut count = vec![0; rmap.len()];
        let mut res = vec![-1; queries.len()];
        let mut cl = 0;
        let mut cr = 0;
        let mut max_i = ndist;
        let mut max_c = -1;
        for (l, r, th, iq) in qs {
            while cl < l {
                let idx = inums[cl];
                count[idx] -= 1;
                if max_i == idx {
                    max_i = ndist;
                    max_c = -1;
                }
                cl += 1;
            }
            while cr > r {
                cr -= 1;
                let idx = inums[cr];
                count[idx] -= 1;
                if max_i == idx {
                    max_i = ndist;
                    max_c = -1;
                }
            }
            while cl > l {
                cl -= 1;
                let idx = inums[cl];
                count[idx] += 1;
                if max_i != ndist {
                    if count[idx] > max_c {
                        max_c = count[idx];
                        max_i = idx;
                    } else if count[idx] == max_c && rmap[idx] < rmap[max_i] {
                        max_i = idx;
                    }
                }
            }
            while cr < r {
                let idx = inums[cr];
                count[idx] += 1;
                if max_i != ndist {
                    if count[idx] > max_c {
                        max_c = count[idx];
                        max_i = idx;
                    } else if count[idx] == max_c && rmap[idx] < rmap[max_i] {
                        max_i = idx;
                    }
                }
                cr += 1;
            }
            if max_i == ndist {
                for i in 0..ndist {
                    if count[i] > max_c {
                        max_i = i;
                        max_c = count[i];
                    } else if count[i] == max_c && rmap[i] < rmap[max_i] {
                        max_i = i;
                    }
                }
            }
            if th <= max_c {
                res[iq] = rmap[max_i];
            }
        }
        res
    }
}