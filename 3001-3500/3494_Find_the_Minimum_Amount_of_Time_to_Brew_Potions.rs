impl Solution {
    pub fn min_time(skill: Vec<i32>, mana: Vec<i32>) -> i64 {
        let n = skill.len();
        let mut ftimes = vec![0i64; n];
        for _m in &mana {
            let m = *_m as i64;
            let mut st = 0i64;
            let mut need = 0;
            for i in 1..n {
                need += skill[i - 1] as i64 * m;
                if st + need < ftimes[i] - ftimes[0] {
                    st = ftimes[i] - ftimes[0] - need;
                }
            }
            ftimes[0] = ftimes[0] + st + skill[0] as i64 * m;
            for i in 1..n {
                ftimes[i] = ftimes[i - 1] + skill[i] as i64 * m;
            }
        }
        ftimes[n - 1]
    }
}