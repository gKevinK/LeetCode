impl Solution {
    fn convert(v: &Vec<i32>, b: i32) -> Vec<i32> {
        let mut d = v.clone();
        let mut q = vec![];
        let mut r = vec![];
        while d.len() > 0 && !d.iter().all(|&x| x == 0) {
            let mut last = 0;
            for i in 0..d.len() {
                let s = last * 10 + d[i];
                last = s % b;
                if s / b > 0 || q.len() > 0 {
                    q.push(s / b);
                }
            }
            r.push(last);
            d.clear();
            (d, q) = (q, d);
        }
        r.reverse();
        r
    }

    pub fn count_numbers(l: String, r: String, b: i32) -> i32 {
        let mut vl: Vec<_> = l.bytes().map(|b| b as i32 - '0' as i32).collect();
        let mut vr: Vec<_> = r.bytes().map(|b| b as i32 - '0' as i32).collect();
        let MOD = 1_000_000_007i64;
        let mut vl = Self::convert(&vl, b);
        let mut vr = Self::convert(&vr, b);

        let mut vc = vec![vec![0; b as usize + 1]; vr.len() + 1];
        let bu = b as usize;
        for ib in 0..b {
            vc[0][ib as usize] = 1;
        }
        for len in 1..vc.len() {
            for ib in 0..b {
                let mut x = 0;
                for ib2 in ib..b {
                    x += vc[len - 1][ib2 as usize];
                }
                vc[len][ib as usize] = x % MOD;
            }
        }

        let mut count = 1i64;
        let mut last = 0;
        for i in 0..vl.len() {
            let len = vl.len() - i;
            if last > vl[i] {
                count += vc[len][last as usize] - 1;
                break;
            }
            last = vl[i];
            count += vc[len][last as usize + 1];
        }
        count %= MOD;
        for len in (vl.len() + 1)..=vr.len() {
            count += vc[len][1];
            count %= MOD;
        }
        last = 0;
        for i in 0..vr.len() {
            let len = vr.len() - i;
            if last > vr[i] {
                count -= vc[len][last as usize];
                break;
            }
            last = vr[i];
            count -= vc[len][last as usize + 1];
        }
        count.rem_euclid(MOD) as i32
    }
}