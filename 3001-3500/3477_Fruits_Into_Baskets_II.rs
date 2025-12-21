impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        let n = fruits.len();
        let mut visit = vec![false; n];
        let mut res = 0;
        for i in 0..n {
            for j in 0..n {
                if visit[j] == false && baskets[j] >= fruits[i] {
                    visit[j] = true;
                    res += 1;
                    break;
                }
            }
        }
        n as i32 - res
    }
}