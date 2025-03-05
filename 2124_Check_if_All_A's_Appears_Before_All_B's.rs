impl Solution {
    pub fn check_string(s: String) -> bool {
        let mut state = 0;
        for ch in s.chars() {
            match state {
                0 => {
                    match ch {
                        'a' => {},
                        'b' => { state = 1 },
                        _ => {},
                    };
                },
                1 => {
                    match ch {
                        'a' => { return false },
                        'b' => {},
                        _ => {},
                    }
                },
                _ => {},
            }
        }
        true
    }
}