impl Solution {
    pub fn generate_string(str1: String, str2: String) -> String {
        let mut s1 = str1.as_bytes();
        let mut s2 = str2.as_bytes();
        let n = str1.len();
        let m = str2.len();
        let len = n + m - 1;
        let mut res = vec![0u8; len];
        let mut fix = vec![false; len];
        for i in 0..n {
            if s1[i] == b'T' {
                for j in 0..m {
                    if fix[i + j] && res[i + j] != s2[j] {
                        return String::new();
                    }
                    res[i + j] = s2[j];
                    fix[i + j] = true;
                }
            }
        }
        for i in 0..len {
            if !fix[i] {
                res[i] = b'a';
            }
        }
        for i in 0..n {
            if s1[i] == b'F' {
                if &res[i..i + m] == s2 {
                    let mut changed = false;
                    for j in (0..m).rev() {
                        let idx = i + j;
                        if !fix[idx] {
                            res[idx] = if s2[j] == b'a' { b'b' } else { b'a' };
                            changed = true;
                            break;
                        }
                    }
                    if !changed {
                        return String::new();
                    }
                }
            }
        }
        for i in 0..n {
            let same = &res[i..i + m] == s2;
            if s1[i] == b'T' && !same {
                return String::new();
            }
            if s1[i] == b'F' && same {
                return String::new();
            }
        }
        String::from_utf8(res).unwrap()
    }
}