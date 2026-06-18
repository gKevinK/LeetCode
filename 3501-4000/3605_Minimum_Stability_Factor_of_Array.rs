impl Solution {
    pub fn min_stable(nums: Vec<i32>, max_c: i32) -> i32 {
        let n = nums.len();
        let mut longest = vec![0; n];
        let mut factor = vec![0; n];
        for i in (0..n).rev() {
            let mut m = nums[i];
            if m >= 2 {
                let mut j = i + 1;
                while j < n {
                    if Self::gcd(m, factor[j]) >= 2 {
                        m = Self::gcd(m, factor[j]);
                        j += longest[j];
                        break;
                    }
                    let m2 = Self::gcd(m, nums[j]);
                    if m2 == 1 {
                        break;
                    }
                    m = m2;
                    j += 1;
                }
                longest[i] = j - i;
                factor[i] = m;
            }
        }

        let test = |len: usize| {
            let mut need = 0;
            let mut i = 0;
            while i < n {
                if longest[i] > len {
                    need += 1;
                    i += len + 1;
                } else {
                    i += 1;
                }
            }
            need <= max_c
        };
        let mut lo = 0;
        let mut hi = n + 1;
        while lo < hi {
            let mi = lo + (hi - lo) / 2;
            if test(mi) {
                hi = mi;
            } else {
                lo = mi + 1;
            }
        }
        lo as _
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        if a < b {
            (a, b) = (b, a);
        }
        while b > 0 {
            let t = a % b;
            a = b;
            b = t;
        }
        a
    }
}