impl Solution {
    pub fn total_numbers(digits: Vec<i32>) -> i32 {
        let mut count = [0; 10];
        for d in digits {
            count[d as usize] += 1;
        }
        let mut res = 0;
        for i1 in 1..=9 {
            if count[i1] == 0 {
                continue;
            }
            count[i1] -= 1;
            for i2 in 0..=9 {
                if count[i2] == 0 {
                    continue;
                }
                count[i2] -= 1;
                for i3 in (0..=9).step_by(2) {
                    if count[i3] > 0 {
                        res += 1;
                    }
                }
                count[i2] += 1;
            }
            count[i1] += 1;
        }
        res
    }
}