impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        use std::collections::VecDeque;
        
        match n {
            0 => 0,
            1 => 1,
            2 => 1,
            _ => {
                let mut v: VecDeque<i32> = vec![0, 1, 1].into_iter().collect();
                for i in 2..n {
                    v.push_back(v.iter().sum());
                    v.pop_front();
                }
                *v.back().unwrap()
            }
        }
    }
}