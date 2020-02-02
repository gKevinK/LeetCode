impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut arr2 = arr.clone();
        arr2.sort();
        let mut map = std::collections::HashMap::new();
        let mut rank = 0;
        for i in 0..arr2.len() {
            if i > 0 && arr2[i] == arr2[i - 1] {
                continue;
            }
            map.insert(arr2[i], rank);
            rank += 1;
        }
        arr.into_iter().map(|i| *map.get(&i).unwrap() + 1).collect()
    }
}