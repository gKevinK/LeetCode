use std::collections::HashMap;

impl Solution {
    pub fn beautiful_numbers(l: i32, r: i32) -> i32 {
        let mut cache = HashMap::new();
        let arr2 = r.to_string().bytes().map(|b| (b - b'0') as i32).collect::<Vec<_>>();
        let l_str = l.to_string();
        let mut arr1 = vec![0; arr2.len() - l_str.len()];
        arr1.extend(l_str.bytes().map(|b| (b - b'0') as i32));
        Self::solve(&mut cache, &arr1, &arr2, 0, 1, 1, 0, 0, 0, 0, 0, 0)
    }

    fn divisible(sum: i32, n2: i32, n3: i32, n5: i32, n7: i32) -> bool {
        let mut prod = 1i64;
        for _ in 0..n2 {
            prod *= 2;
        }
        for _ in 0..n3 {
            prod *= 3;
        }
        for _ in 0..n5 {
            prod *= 5;
        }
        for _ in 0..n7 {
            prod *= 7;
        }
        prod % (sum as i64) == 0
    }

    fn solve(mut cache: &mut HashMap<i32, i32>, arr1: &Vec<i32>, arr2: &Vec<i32>,
            pos: i32, tight1: i32, tight2: i32, sum: i32, n2: i32, n3: i32, n5: i32, n7: i32, zero: i32) -> i32 {
        if pos as usize == arr2.len() {
            if zero > 0 || sum == 0 {
                return 1;
            }
            return Self::divisible(sum, n2, n3, n5, n7) as i32;
        }
        let mut key = (((((((pos * 2 + tight1) * 2 + tight2) * 100 + sum) * 30 + n2) * 20 + n3) * 10 + n5) * 10 + n7) * 2 + zero;
        if zero == 1 {
            key = ((pos * 2 + tight1) * 2 + tight2) * 100 * 30 * 20 * 10 * 10 * 2 + zero;
        }
        if let Some(r) = cache.get(&key) {
            return *r;
        }
        let limit1 = if tight1 == 1 { arr1[pos as usize] } else { 0 };
        let limit2 = if tight2 == 1 { arr2[pos as usize] } else { 9 };
        let mut res = 0;
        for d in limit1..=limit2 {
            let t1 = (tight1 == 1 && d == arr1[pos as usize]) as i32;
            let t2 = (tight2 == 1 && d == arr2[pos as usize]) as i32;
            res += match d {
                0 => Self::solve(&mut cache, &arr1, &arr2, pos + 1, t1, t2, sum + d, n2, n3, n5, n7, (sum > 0) as i32),
                1 => Self::solve(&mut cache, &arr1, &arr2, pos + 1, t1, t2, sum + d, n2, n3, n5, n7, zero),
                2 => Self::solve(&mut cache, &arr1, &arr2, pos + 1, t1, t2, sum + d, n2 + 1, n3, n5, n7, zero),
                3 => Self::solve(&mut cache, &arr1, &arr2, pos + 1, t1, t2, sum + d, n2, n3 + 1, n5, n7, zero),
                4 => Self::solve(&mut cache, &arr1, &arr2, pos + 1, t1, t2, sum + d, n2 + 2, n3, n5, n7, zero),
                5 => Self::solve(&mut cache, &arr1, &arr2, pos + 1, t1, t2, sum + d, n2, n3, n5 + 1, n7, zero),
                6 => Self::solve(&mut cache, &arr1, &arr2, pos + 1, t1, t2, sum + d, n2 + 1, n3 + 1, n5, n7, zero),
                7 => Self::solve(&mut cache, &arr1, &arr2, pos + 1, t1, t2, sum + d, n2, n3, n5, n7 + 1, zero),
                8 => Self::solve(&mut cache, &arr1, &arr2, pos + 1, t1, t2, sum + d, n2 + 3, n3, n5, n7, zero),
                9 => Self::solve(&mut cache, &arr1, &arr2, pos + 1, t1, t2, sum + d, n2, n3 + 2, n5, n7, zero),
                _ => 0
            };
        }
        cache.insert(key, res);
        res
    }
}