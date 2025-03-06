impl Solution {
    pub fn max_rep_opt1(text: String) -> i32 {
        use std::cmp::max;
        use std::cmp::min;
        use std::collections::HashMap;
        
        let t: Vec<char> = text.chars().collect();
        let mut count = HashMap::new();
        for c in &t {
            let e = count.entry(c).or_insert(0);
            *e += 1;
        }
        let mut res = 0;
        let mut len = 0;
        let mut ch = 'a';
        let mut m: HashMap<char, i32> = HashMap::new();
        for c in &t {
            for (k, v) in m.iter_mut() {
                if k == c {
                    *v += 1;
                    res = max(res, min(*v, count[k]));
                } else {
                    *v = 1;
                }
            }
            if ch == *c {
                len += 1;
                res = max(res, min(len, count[c]));
            } else {
                let e = m.entry(ch).or_insert(0);
                *e = len + 1;
                res = max(res, min(len + 1, count[&ch]));
                len = 1;
                ch = *c;
            }
        }
        res
    }
}