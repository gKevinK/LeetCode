impl Solution {
    pub fn min_integer(num: String, k: i32) -> String {
        let mut v = num.as_bytes().to_vec();
        let n = v.len();
        let mut ku = k as usize;
        if n * (n - 1) <= ku {
            v.sort_unstable();
            return String::from_utf8(v).unwrap();
        }
        let mut queue = vec![std::collections::VecDeque::new(); 10];
        for i in 0..n {
            queue[(v[i] - b'0') as usize].push_back(i);
        }
        let mut used = vec![false; n + 1];
        let mut utree = vec![0; n + 1];
        let mut new = Vec::with_capacity(n);
        while ku > 0 && new.len() < n {
            for d in 0..=9 {
                if let Some(&i) = queue[d].front() {
                    let u = Self::get(&utree, i);
                    if i <= ku + u {
                        ku -= i - u;
                        queue[d].pop_front();
                        used[i] = true;
                        Self::add(&mut utree, i);
                        new.push(d as u8 + b'0');
                        break;
                    }
                }
            }
        }
        for i in 0..n {
            if used[i] == false {
                new.push(v[i]);
            }
        }
        String::from_utf8(new).unwrap()
    }

    fn add(mut t: &mut Vec<usize>, mut i: usize) {
        i += 1;
        while i < t.len() {
            t[i] += 1;
            i += i & i.wrapping_neg();
        }
    }

    fn get(t: &Vec<usize>, mut i: usize) -> usize {
        let mut res = 0;
        while i > 0 {
            res += t[i];
            i -= i & i.wrapping_neg();
        }
        res
    }
}