impl Solution {
    pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
        let mut res = vec![0; num_people as usize];
        let mut round = 0;
        let mut rest = candies;
        let tri = num_people * (num_people + 1) / 2;
        let n2 = num_people.pow(2);
        while round * n2 + tri <= rest {
            rest -= round * n2 + tri;
            round += 1;
        }
        let base = round * (round - 1) / 2 * num_people;
        for i in 0..res.len() {
            res[i] = (i as i32 + 1) * round + base;
        }
        {
            let mut i = 0;
            while rest > 0 {
                let t = std::cmp::min(rest, round * num_people + i as i32 + 1);
                res[i] += t;
                rest -= t;
                i += 1;
            }
        }
        res
    }
}