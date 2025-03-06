impl Solution {
    pub fn num_moves_stones_ii(mut stones: Vec<i32>) -> Vec<i32> {
        let n = stones.len() as i32;
        stones.sort();
        let v = stones[(n - 1)as usize] - stones[0];
        let r2 = v + 2 - n - std::cmp::min(stones[1] - stones[0], stones[(n - 1)as usize] - stones[(n - 2)as usize]);
        let mut m = n;
        let mut left = 0;
        for right in 0..n {
            while stones[left as usize] + n - 1 < stones[right as usize] {
                left += 1;
            }
            if right - left + 1 == n - 1 && stones[right as usize] - stones[left as usize] == n - 2 {
                m = std::cmp::min(m, 2);
            } else {
                m = std::cmp::min(m, n - (right - left + 1));
            }
        }
        vec![m, r2]
    }
}