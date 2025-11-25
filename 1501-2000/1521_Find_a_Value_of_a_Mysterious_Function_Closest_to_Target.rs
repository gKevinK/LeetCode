impl Solution {
    pub fn closest_to_target(arr: Vec<i32>, target: i32) -> i32 {
        let n = arr.len();
        let mut mdiff = 1000000000;
        let mut curr = vec![];
        let mut next = vec![];
        let mut left = 0;
        for i in 0..n {
            next.push(arr[i]);
            let mut val = arr[i];
            mdiff = mdiff.min((target - val).abs());
            for &x in &curr {
                if val & x != val {
                    val &= x;
                    let diff = (target - val).abs();
                    mdiff = mdiff.min(diff);
                    if target - mdiff < val {
                        next.push(x);
                    } else {
                        break;
                    }
                }
            }
            curr.clear();
            std::mem::swap(&mut curr, &mut next);
            if mdiff == 0 {
                return 0;
            }
        }
        mdiff
    }
}