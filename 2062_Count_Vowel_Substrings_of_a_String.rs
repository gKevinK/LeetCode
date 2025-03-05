impl Solution {
    pub fn count_vowel_substrings(word: String) -> i32 {
        let chars: Vec<_> = word.chars().collect();
        let mut result = 0;
        let mut left = 0;
        let mut right = 0;
        let to_index = |c| {
            match c {
                'a' => 0,
                'e' => 1,
                'i' => 2,
                'o' => 3,
                'u' => 4,
                _ => -1,
            }
        };
        while left < chars.len() {
            if to_index(chars[left]) == -1 {
                left += 1;
                continue;
            }
            right = left;
            while right < chars.len() && to_index(chars[right]) >= 0 {
                right += 1;
            }
            // println!("{} {}", left, right);
            let mut v = vec![-1; 5];
            for i in left..right {
                let index = to_index(chars[i]);
                if index >= 0 {
                    v[index as usize] = i as i32;
                }
                let m = *v.iter().min().unwrap();
                // println!("{:?}", v);
                if m >= left as i32 {
                    result += m - left as i32 + 1;
                }
            }
            left = right;
        }
        result
    }
}