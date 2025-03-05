impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let mut sum = (n + rolls.len() as i32) * mean - rolls.iter().sum::<i32>();
        if sum < n || n * 6 < sum {
            return vec![];
        }
        let mut v = vec![0; n as usize];
        for i in 0..n {
            v[i as usize] = sum / (n - i);
            sum -= sum / (n - i);
        }
        v
    }
}