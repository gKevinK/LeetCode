impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        let ss = s.as_bytes();
        let n = ss.len();

        let len_one = Self::solve_one(&s);
        let len_ab = Self::solve_two(&s, b'a', b'b', b'c');
        let len_ac = Self::solve_two(&s, b'a', b'c', b'b');
        let len_bc = Self::solve_two(&s, b'b', b'c', b'a');

        let mut state = std::collections::HashMap::new();
        state.insert(0u64, -1);
        let mut max_len = 0;
        let mut diff1 = 0;
        let mut diff2 = 0;
        for i in 0..n {
            let b = ss[i];
            if b == b'a' {
                diff1 += 1;
            } else if b == b'b' {
                diff1 -= 1;
                diff2 += 1;
            } else {
                diff2 -= 1;
            }
            match state.entry((diff1 << 20) + diff2) {
                std::collections::hash_map::Entry::Occupied(e) => {
                    max_len = max_len.max(i as i32 - *e.get());
                },
                std::collections::hash_map::Entry::Vacant(e) => {
                    e.insert(i as i32);
                }
            }
        }
        [max_len, len_one, len_ab, len_bc, len_ac].iter().max().unwrap_or(&0).clone()
    }

    fn solve_one(s: &String) -> i32 {
        let ss = s.as_bytes();
        let n = ss.len();
        let mut left = 0;
        let mut byte = b'd';
        let mut max_len = 0;
        for i in 0..n {
            let b = ss[i];
            if b != byte {
                byte = b;
                left = i;
            }
            max_len = max_len.max((i - left + 1) as i32);
        }
        max_len
    }

    fn solve_two(s: &String, b1: u8, b2: u8, drop: u8) -> i32 {
        let ss = s.as_bytes();
        let n = ss.len();
        let mut first = vec![-1; 2 * n + 1];
        first[n] = 0;
        let mut max_len = 0;
        let mut diff = n;
        let mut start = 0;
        for i in 0..n {
            let b = ss[i];
            if b == drop {
                start = i as i32 + 1;
                diff = n;
                first[n] = i as i32 + 1;
                continue;
            }
            if b == b1 {
                diff += 1;
            } else {
                diff -= 1;
            }
            if first[diff] >= start {
                max_len = max_len.max(i as i32 + 1 - first[diff]);
            } else {
                first[diff] = i as i32 + 1;
            }
        }
        max_len
    }
}