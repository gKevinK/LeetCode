impl Solution {
    pub fn decode_ciphertext(encoded_text: String, rows: i32) -> String {
        let encoded: Vec<_> = encoded_text.chars().collect();
        let nr = rows as usize;
        let nc = encoded_text.len() / nr;
        let mut text = String::new();
        let available = |i| { (i % nr + i / nr) < nc };
        let pos = |i| { (i % nr) * nc + (i % nr + i / nr) };
        let mut i = 0;
        loop {
            let mut j = i;
            let mut f = true;
            loop {
                if j >= encoded_text.len() {
                    f = false;
                    break;
                }
                if !available(j) {
                    j += 1;
                    continue;
                }
                if encoded[pos(j)] != ' ' {
                    j += 1;
                    break;
                }
                j += 1;
            }
            if f {
                for k in i..j {
                    if !available(k) {
                        continue;
                    }
                    text.push(encoded[pos(k)]);
                }
                i = j;
            } else {
                break;
            }
        }
        text
    }
}