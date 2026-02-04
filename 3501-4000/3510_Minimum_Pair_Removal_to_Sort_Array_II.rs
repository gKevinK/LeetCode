impl Solution {
    pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut v: Vec<_> = nums.iter().map(|x| *x as i64).collect();
        let mut pairs = std::collections::BTreeSet::new();
        let mut prev = vec![0; n];
        let mut next = vec![0; n];
        for i in 0..n {
            prev[i] = i - 1;
            next[i] = i + 1;
        }

        let mut count = 0;
        for i in 0..(n - 1) {
            if v[i] > v[i + 1] {
                count += 1;
            }
            pairs.insert((v[i] + v[i + 1], i));
        }

        let mut res = 0;
        while count > 0 {
            let (sum, i) = pairs.pop_first().unwrap();
            let j = next[i];
            let p = prev[i];
            let q = next[j];

            if v[i] > v[j] {
                count -= 1;
            }
            if p < n {
                count -= i32::from(v[p] > v[i]);
                count += i32::from(v[p] > sum);
                pairs.remove(&(v[p] + v[i], p));
                pairs.insert((v[p] + sum, p));
            }
            if q < n {
                count -= i32::from(v[j] > v[q]);
                count += i32::from(sum > v[q]);
                pairs.remove(&(v[j] + v[q], j));
                pairs.insert((sum + v[q], i));
                prev[q] = i;
            }
            next[i] = q;
            v[i] = sum;
            res += 1;
        }
        res
    }
}