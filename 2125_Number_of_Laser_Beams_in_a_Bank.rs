impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let mut last = 0;
        let mut lb = 0;
        for b in bank {
            let n = b.chars().filter(|x| *x == '1').count() as i32;
            if n > 0 {
                lb += last * n;
                last = n;
            }
        }
        lb
    }
}