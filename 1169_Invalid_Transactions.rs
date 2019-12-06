impl Solution {
    pub fn invalid_transactions(transactions: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;
        let mut m: HashMap<&str, Vec<(i32, &str)>> = HashMap::new();
        for t in &transactions {
            let l: Vec<&str> = t.split(',').collect();
            let e = m.entry(l[0]).or_insert(Vec::new());
            (*e).push((l[1].parse().unwrap(), l[3]));
        }
        for (_, v) in &mut m {
            v.sort();
        }
        let mut r: Vec<String> = Vec::new();
        for t in &transactions {
            let l: Vec<&str> = t.split(',').collect();
            if l[2].parse::<i32>().unwrap() > 1000 {
                r.push(t.to_string());
            } else {
                let &e = m.get(l[0]).as_ref().unwrap();
                let time = l[1].parse::<i32>().unwrap();
                let mut lo = 0;
                let mut hi = e.len() - 1;
                while lo < hi {
                    let mi = (hi - lo) / 2 + lo;
                    if e[mi].0 < time {
                        lo = mi + 1;
                    } else if e[mi].0 > time {
                        hi = mi - 1;
                    } else {
                        lo = mi;
                        hi = mi;
                    }
                }
                let mut i = lo;
                let mut flag = true;
                while flag && i < e.len() && e[i].0 >= time - 60 {
                    if e[i].1 != l[3] {
                        flag = false;
                    }
                    i -= 1;
                }
                i = lo + 1;
                while flag && i < e.len() && e[i].0 <= time + 60 {
                    if e[i].1 != l[3] {
                        flag = false;
                    }
                    i += 1;
                }
                if !flag {
                    r.push(t.to_string());
                }
            }
        }
        r
    }
}