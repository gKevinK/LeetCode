impl Solution {
    pub fn score(cards: Vec<String>, x: char) -> i32 {
        let mut c1 = [0; 26];
        let mut c2 = [0; 26];
        for card in &cards {
            let card = card.as_bytes();
            if card[0] == x as u8 {
                c1[(card[1] - b'a') as usize] += 1;
            }
            if card[1] == x as u8 {
                c2[(card[0] - b'a') as usize] += 1;
            }
        }
        let mut c1_max = 0;
        let mut c1_n = 0;
        let mut c2_max = 0;
        let mut c2_n = 0;
        for i in 0..26 {
            if i != x as usize - 'a' as usize {
                c1_max = c1_max.max(c1[i]);
                c1_n += c1[i];
                c2_max = c2_max.max(c2[i]);
                c2_n += c2[i];
            }
        }
        let xx = c1[x as usize - 'a' as usize];
        let mut res = 0;
        let mut need1 = 0.max(c1_max - (c1_n - c1_max));
        if need1 > 0 {
            res += c1_n - c1_max;
        } else {
            res += c1_n / 2;
            need1 = c1_n % 2;
        }
        let mut need2 = 0.max(c2_max - (c2_n - c2_max));
        if need2 > 0 {
            res += c2_n - c2_max;
        } else {
            res += c2_n / 2;
            need2 = c2_n % 2;
        }
        if xx <= need1 + need2 {
            res += xx;
        } else {
            res += need1 + need2 + ((xx - need1 - need2) / 2).min(res);
        }
        res
    }
}