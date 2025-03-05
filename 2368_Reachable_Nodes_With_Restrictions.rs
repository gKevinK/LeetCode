impl Solution {
    pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        use std::collections::HashSet;
        use std::collections::VecDeque;
        let restricted_set: HashSet<_> = restricted.into_iter().collect();
        let mut neighbors: HashMap<i32, Vec<i32>> = HashMap::new();
        for e in edges {
            neighbors.entry(e[0]).or_insert(Vec::new()).push(e[1]);
            neighbors.entry(e[1]).or_insert(Vec::new()).push(e[0]);
        }
        let mut visited = HashSet::from([0]);
        let mut queue = VecDeque::from([0]);
        while let Some(c) = queue.pop_front() {
            for &n in &neighbors[&c] {
                if visited.contains(&n) {
                    continue;
                }
                if restricted_set.contains(&n) {
                    continue;
                }
                queue.push_back(n);
                visited.insert(n);
            }
        }
        visited.len() as i32
    }
}