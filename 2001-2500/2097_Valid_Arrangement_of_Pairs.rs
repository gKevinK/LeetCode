impl Solution {

    fn dfs(mut edge: &mut std::collections::HashMap<i32, Vec<i32>>, mut res: &mut Vec<Vec<i32>>, s: i32) {
        while let Some(next) = edge.get_mut(&s).unwrap().pop() {
            Self::dfs(&mut edge, &mut res, next);
            res.push(vec![s, next]);
        }
    }

    pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::HashMap;
        let n = pairs.len();
        let mut eout: HashMap<i32, i32> = HashMap::new();
        let mut edge = HashMap::new();
        for p in &pairs {
            *eout.entry(p[0]).or_insert(0) += 1;
            *eout.entry(p[1]).or_insert(0) -= 1;
            edge.entry(p[0]).or_insert(vec![]).push(p[1]);
            edge.entry(p[1]).or_insert(vec![]);
        }
        let mut start = -1;
        for i in eout.keys() {
            if eout[i] == 1 {
                start = *i;
                break;
            }
        }
        if start == -1 {
            start = pairs[0][0];
        }
        let mut res = vec![];
        Self::dfs(&mut edge, &mut res, start);
        res.reverse();
        res
    }
}