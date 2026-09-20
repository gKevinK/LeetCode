impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        let ss = s.as_bytes();
        let n = ss.len();
        let mut res = 0;
        for i in 0..n {
            let mut count = [0; 26];
            let mut cnum = 0;
            let mut cmax = 0;
            for j in i..n {
                let mut ncount = &mut count[(ss[j] - b'a') as usize];
                *ncount += 1;
                if *ncount == 1 {
                    cnum += 1;
                }
                cmax = cmax.max(*ncount);
                if (j - i + 1) == cnum * cmax {
                    res = res.max(j - i + 1);
                }
            }
        }
        res as _
    }
}