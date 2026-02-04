impl Solution {
    const MIN_PROD: [i32; 41] = [1,
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        11, 12, 24, 36, 48, 60, 72, 84, 96, 108,
        120, 132, 144, 288, 432, 576, 720, 864, 1008, 1152,
        1296, 1440, 1584, 1728, 3456, 5184, 6912, 8640, 10368, 12096];

    pub fn max_product(nums: Vec<i32>, k: i32, limit: i32) -> i32 {
        let mut res = -1;
        let mut set = std::collections::HashSet::new();
        set.insert((1, 0, 1));
        let mut qadd = vec![];
        // Try subsequences without zero
        for i in 0..nums.len() {
            if nums[i] == 0 {
                continue;
            }
            for &(sign, asum, prod) in &set {
                let asum2 = asum + sign * nums[i];
                let sign2 = -sign;
                let prod2 = (prod * nums[i]).min(limit + 1);
                if asum2 == k && prod2 <= limit {
                    res = res.max(prod2);
                }
                let rest = (k - asum2).abs() + if sign2 * (k - asum2) >= 0 { 0 } else { 1 };
                let rest_prod = Self::MIN_PROD[rest.min(40) as usize];
                if prod2 * rest_prod <= limit {
                    qadd.push((sign2, asum2, prod2));
                }
            }
            for x in qadd.drain(..) {
                set.insert(x);
            }
        }
        // Try subsequences with zero
        if res == -1 {
            set.clear();
            set.insert((1, 0, 1));
            for i in 0..nums.len() {
                for &(sign, asum, prod) in &set {
                    let asum2 = asum + sign * nums[i];
                    let sign2 = -sign;
                    let prod2 = prod * if nums[i] > 0 { 1 } else { 0 };
                    if asum2 == k && prod2 == 0 {
                        return 0;
                    }
                    qadd.push((sign2, asum2, prod2));
                }
                for x in qadd.drain(..) {
                    set.insert(x);
                }
            }
        }
        res
    }
}