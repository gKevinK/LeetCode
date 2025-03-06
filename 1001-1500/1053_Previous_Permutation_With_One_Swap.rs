impl Solution {
    pub fn prev_perm_opt1(a: Vec<i32>) -> Vec<i32> {
        let mut r = a.clone();
        let mut s = Vec::new();
        for i in (0..a.len()).rev() {
            let mut j = 0;
            while let Some(l) = s.pop() {
                if a[i] <= a[l] {
                    s.push(l);
                    break;
                }
                j = l;
            }
            if j > 0 && a[i] != a[j] {
                r[j] = a[i];
                r[i] = a[j];
                return r;
            }
            while let Some(l) = s.pop() {
                if a[i] < a[l] {
                    s.push(l);
                    break;
                }
            }
            s.push(i);
        }
        r
    }
}