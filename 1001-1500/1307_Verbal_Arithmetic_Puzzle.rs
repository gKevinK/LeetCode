use std::collections::HashMap;
impl Solution {
    fn f(words: &Vec<Vec<char>>, result: &Vec<char>, mut c2i: &mut HashMap<char, i32>, mut iv: &mut Vec<bool>, ic: usize, iw: usize, is: i32) -> bool {
        if ic == result.len() {
            return is == 0;
        }
        if iw == words.len() {
            if !c2i.contains_key(&result[ic]) && !iv[(is % 10) as usize] {
                if is % 10 == 0 && ic + 1 == result.len() {
                    return false;
                }
                c2i.insert(result[ic], is % 10);
                iv[(is % 10) as usize] = true;
                let t = Self::f(&words, &result, &mut c2i, &mut iv, ic + 1, 0, is / 10);
                c2i.remove(&result[ic]);
                iv[(is % 10) as usize] = false;
                return t;
            } else if c2i.get(&result[ic]) == Some(&(is % 10)) {
                return Self::f(&words, &result, &mut c2i, &mut iv, ic + 1, 0, is / 10);
            } else {
                return false;
            }
        }
        if ic >= words[iw].len() {
            return Self::f(&words, &result, &mut c2i, &mut iv, ic, iw + 1, is);
        }
        if let Some(&i) = c2i.get(&words[iw][ic]) {
            if ic > 0 && ic + 1 == words[iw].len() && i == 0 {
                return false;
            }
            return Self::f(&words, &result, &mut c2i, &mut iv, ic, iw + 1, is + i);
        }
        let start = if ic > 0 && ic + 1 == words[iw].len() { 1 } else { 0 };
        for i in start..=9 {
            if iv[i as usize] {
                continue;
            }
            c2i.insert(words[iw][ic], i);
            iv[i as usize] = true;
            let t = Self::f(&words, &result, &mut c2i, &mut iv, ic, iw + 1, is + i);
            c2i.remove(&words[iw][ic]);
            iv[i as usize] = false;
            if t {
                return true;
            }
        }
        false
    }
    pub fn is_solvable(words: Vec<String>, result: String) -> bool {
        let ws: Vec<Vec<char>> = words.iter().map(|w| w.chars().rev().collect()).collect();
        let rs: Vec<char> = result.chars().rev().collect();
        let mut c2i = HashMap::new();
        let mut iv = vec![ false; 10 ];
        if ws.iter().map(|w| w.len()).max().unwrap() > rs.len() {
            return false;
        }
        Self::f(&ws, &rs, &mut c2i, &mut iv, 0, 0, 0)
    }
}