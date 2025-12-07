impl Solution {
    pub fn count_components(mut nums: Vec<i32>, threshold: i32) -> i32 {
        let mut res = 0;
        nums.sort_unstable();
        while let Some(x) = nums.last() {
            if *x > threshold {
                res += 1;
                nums.pop();
            } else {
                break;
            }
        }

        let n = nums.len();
        let t = threshold as usize;
        let mut dsu = (0..t + 1).map(|x| (x, 0)).collect::<Vec<_>>();
        for &num in &nums {
            let num = num as usize;
            for m in ((num * 2)..=t).step_by(num) {
                Self::unite(&mut dsu, num, m);
            }
        }

        let mut set = std::collections::HashSet::new();
        for &num in &nums {
            set.insert(Self::find(&mut dsu, num as usize));
        }
        res + set.len() as i32
    }

    fn find(mut dsu: &mut Vec<(usize, i32)>, i: usize) -> usize {
        let mut r = i;
        while dsu[r].0 != r {
            r = dsu[r].0;
        }
        dsu[i].0 = r;
        r
    }

    fn unite(mut dsu: &mut Vec<(usize, i32)>, a: usize, b: usize) {
        let ia = Self::find(&mut dsu, a);
        let ib = Self::find(&mut dsu, b);
        if ia != ib {
            if dsu[ia].1 > dsu[ib].1 {
                dsu[ib].0 = ia;
                dsu[ia].1 = dsu[ia].1.max(dsu[ib].1 + 1);
            } else {
                dsu[ia].0 = ib;
                dsu[ib].1 = dsu[ib].1.max(dsu[ia].1 + 1);
            }
        }
    }
}