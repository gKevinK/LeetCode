impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>, swaps: Vec<Vec<i32>>) -> i64 {
        let n = nums.len();
        let mut dsu = (0..n).map(|x| (x, 0)).collect::<Vec<_>>();
        for swap in &swaps {
            Self::unite(&mut dsu, swap[0] as usize, swap[1] as usize);
        }
        let mut group = vec![(0, vec![]); n];
        for i in 0..n {
            let p = Self::find(&mut dsu, i);
            group[p].1.push(nums[i] as i64);
            if i % 2 == 1 {
                group[p].0 += 1;
            }
        }
        let mut res = 0;
        for (odd, mut gnums) in group {
            if odd < gnums.len() {
                let (lo, mi, hi) = gnums.select_nth_unstable(odd);
                for num in lo {
                    res -= *num;
                }
                res += *mi;
                for num in hi {
                    res += *num;
                }
            } else {
                for num in gnums {
                    res -= num;
                }
            }
        }
        res
    }

    fn find(mut dsu: &mut Vec<(usize, i32)>, i: usize) -> usize {
        if dsu[i].0 == i {
            return i;
        }
        let p = dsu[i].0;
        dsu[i].0 = Self::find(&mut dsu, p);
        dsu[i].0
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