impl Solution {
    pub fn max_capacity(costs: Vec<i32>, capacity: Vec<i32>, budget: i32) -> i32 {
        let n = costs.len();
        let mut machines = (0..n).map(|i| (costs[i], capacity[i])).collect::<Vec<_>>();
        machines.sort_unstable();
        let mut prefix_max = vec![0; n];
        let mut max = 0;
        let mut j = n;
        let mut res = 0;
        for i in 0..n {
            let (cost, cap) = machines[i];
            max = max.max(cap);
            prefix_max[i] = max;

            if cost < budget {
                res = res.max(cap);
            }
            while j > 0 && machines[j - 1].0 + cost >= budget {
                j -= 1;
            }
            let l = i.min(j);
            if l > 0 {
                res = res.max(cap + prefix_max[l - 1]);
            }
        }
        res
    }
}