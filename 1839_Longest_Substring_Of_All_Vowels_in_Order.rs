impl Solution {
    pub fn longest_beautiful_substring(word: String) -> i32 {
        let mut len = 0;
        let mut mlen = 0;
        let mut state = 0;
        for c in word.chars() {
            match c {
                'a' => {
                    if state == 1 {
                        len += 1;
                    } else {
                        state = 1;
                        len = 1;
                    }
                },
                'e' => {
                    if state == 1 {
                        state = 2;
                    }
                    if state == 2 {
                        len += 1;
                    } else {
                        state = 0;
                        len = 0;
                    }
                },
                'i' => {
                    if state == 2 {
                        state = 3;
                    }
                    if state == 3 {
                        len += 1;
                    } else {
                        state = 0;
                        len = 0;
                    }
                },
                'o' => {
                    if state == 3 {
                        state = 4;
                    }
                    if state == 4 {
                        len += 1;
                    } else {
                        state = 0;
                        len = 0;
                    }
                },
                'u' => {
                    if state == 4 {
                        state = 5;
                    }
                    if state == 5 {
                        len += 1;
                        mlen = std::cmp::max(mlen, len);
                    } else {
                        state = 0;
                        len = 0;
                    }
                },
                _ => {
                    state = 0;
                    len = 0;
                }
            }
        }
        mlen
    }
}