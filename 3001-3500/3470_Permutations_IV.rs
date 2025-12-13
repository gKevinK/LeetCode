impl Solution {
    pub fn permute(n: i32, mut k: i64) -> Vec<i32> {
        let mut fact = vec![1];
        for i in 1..=(n / 2 + 1) {
            fact.push(fact.last().unwrap() * i as i64);
            if *fact.last().unwrap() > std::cmp::min(k, 1i64 << 30)  {
                break;
            }
        }
        let mut used = vec![false; n as usize + 1];
        let mut is_odd = false;
        let mut res = vec![];

        // Select first integer if both odd and even are possible
        let mut this = 1;
        if n % 2 == 0 {
            let e = n as usize / 2;
            if e < fact.len() {
                let rest = fact[e] * fact[e - 1];
                this = (k - 1) / rest + 1;
                k = (k - 1) % rest + 1;
            }
            is_odd = this % 2 == 1;
            if this as i32 > n {
                return vec![];
            }
            used[this as usize] = true;
            res.push(this as i32);
        }

        // Calculate the rest
        while res.len() < n as usize {
            is_odd = !is_odd;
            let mut this_idx = 1;
            let rest_n = n as usize - res.len() - 1;
            let o = rest_n / 2 + if rest_n % 2 == 1 && !is_odd { 1 } else { 0 };
            let e = rest_n - o;
            if o < fact.len() && e < fact.len() {
                let rest = fact[o] * fact[e];
                this_idx = (k - 1) / rest + 1;
                k = (k - 1) % rest + 1;
            }
            if this_idx > rest_n as i64 / 2 + 1 {
                return vec![];
            }
            for i in ((if is_odd { 1 } else { 2 })..used.len()).step_by(2) {
                if used[i] == false {
                    this_idx -= 1;
                    if this_idx == 0 {
                        used[i] = true;
                        res.push(i as i32);
                        break;
                    }
                }
            }
        }
        res
    }
}