impl Solution {
    pub fn min_arrivals_to_discard(arrivals: Vec<i32>, w: i32, m: i32) -> i32 {
        let n = arrivals.len();
        let mut discard = vec![false; n];
        let mut num = vec![0; *arrivals.iter().max().unwrap() as usize + 1];
        let mut res = 0;
        for i in 0..n {
            if w as usize <= i {
                let j = i - w as usize;
                if discard[j] == false {
                    num[arrivals[j] as usize] -= 1;
                }
            }
            if num[arrivals[i] as usize] < m {
                num[arrivals[i] as usize] += 1;
            } else {
                discard[i] = true;
                res += 1;
            }
        }
        res
    }
}