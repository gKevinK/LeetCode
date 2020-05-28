impl Solution {
    fn f(m: &Vec<Vec<i32>>, h: &Vec<bool>, n: i32, from: i32) -> i32 {
        let mut r = 0;
        for &next in &m[n as usize] {
            if next == from {
                continue;
            }
            r += Self::f(&m, &h, next, n);
        }
        if r > 0 || h[n as usize] { r + 2 } else { r }
    }
    pub fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
        let mut m = vec![vec![]; n as usize];
        for e in edges {
            m[e[0] as usize].push(e[1]);
            m[e[1] as usize].push(e[0]);
        }
        std::cmp::max(0, Self::f(&m, &has_apple, 0, -1) - 2)
    }
}