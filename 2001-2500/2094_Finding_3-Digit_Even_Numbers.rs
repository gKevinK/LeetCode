impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut count = vec![0; 10];
        for d in digits {
            count[d as usize] += 1;
        }
        let mut set = std::collections::HashSet::new();
        for i in 1..=9 {
            if count[i] == 0 {
                continue;
            }
            count[i] -= 1;
            for j in 0..=9 {
                if count[j] == 0 {
                    continue;
                }
                count[j] -= 1;
                for k in 0..=9 {
                    if count[k] == 0 || k % 2 == 1 {
                        continue;
                    }
                    if count[k] >= 1 {
                        set.insert((i * 100 + j * 10 + k) as i32);
                    }
                }
                count[j] += 1;
            }
            count[i] += 1;
        }
        let mut result: Vec<_> = set.into_iter().collect();
        result.sort();
        result
    }
}