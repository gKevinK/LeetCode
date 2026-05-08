impl Solution {
    pub fn concatenated_divisibility(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
        let ku = k as usize;
        let n = nums.len();
        nums.sort();
        let mut nlen = vec![1; n];
        for i in 0..n {
            let mut x = nums[i];
            while x > 0 {
                nlen[i] *= 10;
                x /= 10;
            }
        }
        let ALL = (1 << n) - 1;
        let MAX = 100;
        let mut dp = vec![true; (1 << n) * ku];
        let mut res = vec![MAX; n];
        let mut state = vec![((0, 0), vec![]); n];
        for x in (0..n).rev() {
            state[0].1.push(x);
        }
        let mut i = 0;
        loop {
            let (used, rem) = state[i].0;
            if state[i].1.len() == 0 {
                dp[(ALL ^ used) * ku + rem as usize] = false;
                res[i] = MAX;
                if i == 0 {
                    break;
                }
                i -= 1;
            } else {
                let next = state[i].1.pop().unwrap();
                res[i] = next;
                let next_used = used | (1 << next);
                let next_rem = (rem * nlen[next] + nums[next]) % k;
                if next_used == ALL {
                    if next_rem == 0 {
                        return res.iter().map(|x| nums[*x]).collect();
                    }
                    res[i] = MAX;
                } else if dp[(ALL ^ next_used) * ku + next_rem as usize] == false {
                    res[i] = MAX;
                } else {
                    i += 1;
                    state[i].0 = (next_used, next_rem);
                    for x in (0..n).rev() {
                        if next_used & (1 << x) == 0 {
                            state[i].1.push(x);
                        }
                    }
                }
            }
        }
        Vec::new()
    }
}