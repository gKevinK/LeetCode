impl Solution {
    pub fn map_word_weights(words: Vec<String>, weights: Vec<i32>) -> String {
        let n = words.len();
        let mut res = vec![0u8; n];
        for (i, word) in words.iter().enumerate() {
            let sum: i32 = word.bytes().map(|b| weights[(b - b'a') as usize]).sum();
            res[i] = b'z' - (sum % 26) as u8;
        }
        String::from_utf8(res).unwrap()
    }
}