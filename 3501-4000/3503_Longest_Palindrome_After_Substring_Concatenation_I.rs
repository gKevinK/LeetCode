impl Solution {
    fn longest_p(v: &Vec<u8>, i: usize) -> usize {
        for r in (i..=v.len()).rev() {
            let mut flag = true;
            for k in i..r {
                if k - i >= r - k {
                    break;
                }
                if v[k] != v[r - 1 - (k - i)] {
                    flag = false;
                    break;
                }
            }
            if flag {
                return r - i;
            }
        }
        0
    }

    pub fn longest_palindrome(s: String, t: String) -> i32 {
        let mut res = 0;
        let vs = s.bytes().collect::<Vec<_>>();
        let vt = t.bytes().rev().collect::<Vec<_>>();
        for it in 0..vt.len() {
            let mut i = 0;
            while i < vs.len() {
                let (mut j1, mut j2) = (i, it);
                while j1 < vs.len() && j2 < vt.len() && vs[j1] == vt[j2] {
                    j1 += 1; j2 += 1;
                }
                let mut plen1 = Self::longest_p(&vs, j1);
                let mut plen2 = Self::longest_p(&vt, j2);
                res = res.max(((j1 - i) * 2 + plen1.max(plen2)) as i32);
                i += 1;
            }
        }
        res
    }
}