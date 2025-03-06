impl Solution {
    const fn prime_map<const N: usize>() -> [bool; N] {
        let mut p = [true; N];
        p[0] = false;
        p[1] = false;
        let mut i = 2;
        while i < N {
            if p[i] {
                let mut j = 2 * i;
                while j < N {
                    p[j] = false;
                    j += i;
                }
            }
            i += 1;
        }
        p
    }

    const P: [bool; 10001] = Self::prime_map::<10001>();

    pub fn min_operations(n: i32, m: i32) -> i32 {
        if Self::P[n as usize] || Self::P[m as usize] {
            return -1;
        }
        let mut l = 1;
        while n >= l {
            l *= 10;
        }
        let mut v = vec![];
        v.extend_from_slice(&Self::P[0..l as usize]);
        use std::cmp::Reverse as R;
        let mut q = std::collections::BinaryHeap::new();
        q.push((R(n), n));
        while let Some((R(s), c)) = q.pop() {
            if v[c as usize] {
                continue;
            }
            v[c as usize] = true;
            if c == m {
                return s;
            }
            let mut mask = 1;
            while c / mask > 0 {
                let d = c / mask % 10;
                if d < 9 {
                    let next = c + mask;
                    if !Self::P[next as usize] && !v[next as usize] {
                        q.push((R(s + next), next));
                    }
                }
                if d > 0 && (c / mask != 1) {
                    let next = c - mask;
                    if !Self::P[next as usize] && !v[next as usize] {
                        q.push((R(s + next), next));
                    }
                }
                mask *= 10;
            }
        }
        -1
    }
}