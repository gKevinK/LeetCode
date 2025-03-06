impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        let mut iw = 1;
        let vw: Vec<_> = search_word.chars().collect();
        let vs: Vec<_> = sentence.chars().collect();
        let mut i = 0;
        while i < vs.len() {
            let mut j = 0;
            while i + j < vs.len() && j < vw.len() {
                if vs[i + j] != vw[j] {
                    j = 0;
                    break;
                }
                j += 1;
            }
            if j == 0 {
                while i < vs.len() && vs[i] != ' ' {
                    i += 1;
                }
                i += 1;
                iw += 1;
            } else {
                return iw as i32;
            }
        }
        -1
    }
}