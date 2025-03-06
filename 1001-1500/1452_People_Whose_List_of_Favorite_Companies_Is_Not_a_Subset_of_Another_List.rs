impl Solution {
    pub fn people_indexes(favorite_companies: Vec<Vec<String>>) -> Vec<i32> {
        use std::collections::HashMap;
        // use std::collections::BitSet;
        use std::cmp::Reverse;
        let mut c2i = HashMap::new();
        for v in &favorite_companies {
            for c in v {
                if !c2i.contains_key(c) {
                    c2i.insert(c, c2i.len() as i32);
                }
            }
        }
        let l = (c2i.len() as i32 + 31) / 32;
        let mut persons: Vec<_> = favorite_companies.iter().enumerate().map(|(i, v)| {
            let mut m = vec![0u32; l as usize];
            for c in v {
                let r = c2i.get(c).unwrap();
                m[(r / 32) as usize] |= 1 << r % 32;
            }
            (Reverse(v.len()), i as i32, m)
        }).collect();
        persons.sort_unstable();
        let mut vm = Vec::new();
        let mut r = Vec::new();
        for p in persons {
            if !vm.iter().any(|m: &Vec<_>| m.iter().zip(p.2.iter()).all(|(a, b)| a & b == *b)) {
                vm.push(p.2);
                r.push(p.1);
            }
        }
        r.sort();
        r
    }
