impl Solution {
    fn get_p_len(v: &Vec<u8>) -> Vec<usize> {
        let mut p = vec![0; v.len() + 1];
        for i in 0..v.len() {
            p[i] = 1;
            let mut i1 = i;
            let mut i2 = i;
            while i1 > 0 && i2 + 1 < v.len() {
                i1 -= 1; i2 += 1;
                if v[i1] != v[i2] {
                    break;
                }
                p[i1] = p[i1].max(i2 - i1 + 1);
            }
            i1 = i + 1;
            i2 = i;
            while i1 > 0 && i2 + 1 < v.len() {
                i1 -= 1; i2 += 1;
                if v[i1] != v[i2] {
                    break;
                }
                p[i1] = p[i1].max(i2 - i1 + 1);
            }
        }
        p
    }

    pub fn longest_palindrome(s: String, t: String) -> i32 {
        let mut res = 0;
        let vs = s.bytes().collect::<Vec<_>>();
        let vt = t.bytes().rev().collect::<Vec<_>>();
        let mut p1 = Self::get_p_len(&vs);
        let mut p2 = Self::get_p_len(&vt);

        for it in 0..vt.len() {
            let mut i = 0;
            while i < vs.len() {
                let (mut j1, mut j2) = (i, it);
                while j1 < vs.len() && j2 < vt.len() && vs[j1] == vt[j2] {
                    j1 += 1; j2 += 1;
                }
                let mut plen1 = p1[j1];
                let mut plen2 = p2[j2];
                res = res.max(((j1 - i) * 2 + plen1.max(plen2)) as i32);
                i += 1;
            }
        }
        res
    }
}