impl Solution {
    pub fn max_ice_cream(mut costs: Vec<i32>, mut coins: i32) -> i32 {
        costs.sort();
        let mut num = 0;
        for i in costs {
            if coins >= i {
                num += 1;
                coins -= i;
            } else {
                break;
            }
        }
        num
    }
}