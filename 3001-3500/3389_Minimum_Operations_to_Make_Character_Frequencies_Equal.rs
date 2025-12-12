impl Solution {
    pub fn make_string_good(s: String) -> i32 {
        let mut count = [0; 26];
        for c in s.bytes() {
            count[c as usize - 'a' as usize] += 1;
        }
        let MAX = 20000;
        let mut res = MAX;
        for t in 0..=*count.iter().max().unwrap() {
            let mut op1 = 0;
            let mut op2 = MAX;
            let mut save = 0;
            for i in 0..26 {
                if count[i] >= t {
                    op1 = op1 + count[i] - t;
                    op2 = op1;
                    save = count[i] - t;
                } else {
                    let cand1 = std::cmp::min(op1 + t - count[i], op2 + 0.max(t - count[i] - save));
                    let cand2 = op1 + count[i];
                    op1 = cand1.min(cand2);
                    op2 = cand2;
                    save = count[i];
                }
            }
            res = res.min(op1);
        }
        res
    }
}