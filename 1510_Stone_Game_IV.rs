impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        let mut dp = vec![false];
        for i in 1..=n {
            let mut f = false;
            for j in 1..=i {
                if j * j > i {
                    break;
                }
                if dp[(i - j * j) as usize] == false {
                    f = true;
                    break;
                }
            }
            dp.push(f);
        }
        *dp.last().unwrap()
    }
}