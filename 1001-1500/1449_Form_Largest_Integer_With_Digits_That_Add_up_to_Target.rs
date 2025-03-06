impl Solution {
    pub fn largest_number(cost: Vec<i32>, mut target: i32) -> String {
        let mut v = vec![-1; target as usize + 1];
        v[0] = 0;
        for i in 0..target as usize {
            if v[i] < 0 {
                continue;
            }
            for c in &cost {
                let r = i + *c as usize;
                if r < v.len() {
                    v[r] = std::cmp::max(v[r], v[i] + 1);
                }
            }
        }
        if v[target as usize] == -1 {
            return String::from("0");
        }
        let mut s = String::from("");
        for j in (0..9).rev() {
            while target >= cost[j] && v[(target - cost[j]) as usize] + 1 == v[target as usize] {
                s.push(('1' as u8 + j as u8) as char);
                target -= cost[j];
            }
        }
        s
    }
}