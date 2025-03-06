impl Solution {
    pub fn max_profit(mut inventory: Vec<i32>, mut orders: i32) -> i32 {
        let MOD = 1_000_000_007i64;
        let mut res = 0i64;
        let n = inventory.len();
        inventory.sort();
        let mut i = n - 1;
        while orders > 0 {
            while i > 0 && inventory[i - 1] == inventory[i] {
                i -= 1;
            }
            let l = if i > 0 { inventory[i - 1] } else { 0 } as i64;
            let r = inventory[i] as i64;
            let k = (n - i) as i64;
            let c = (r - l) * k;
            if c >= orders as i64 {
                let d = orders as i64 / k;
                res += (r - d + r + 1) * d / 2 * k + (orders as i64 % k) * (r - d);
                res %= MOD;
                orders = 0;
            } else {
                res += (l + r + 1) * (r - l) / 2 * k;
                res %= MOD;
                orders -= c as i32;
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }
        (res % MOD) as i32
    }
}