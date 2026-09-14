impl Solution {
    pub fn majority_frequency_group(s: String) -> String {
        let mut freq = [0; 26];
        for b in s.bytes() {
            freq[(b - b'a') as usize] += 1;
        }
        let mut group = [0; 101];
        for i in 0..26 {
            group[freq[i]] += 1;
        }
        let mut max = 100;
        for i in (1..100).rev() {
            if group[i] > group[max] {
                max = i;
            }
        }
        (0..26).filter(|&u| freq[u] == max).map(|u| (u as u8 + b'a') as char).collect()
    }
}