impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut count = vec![0; arr2.len()];
        let mut map = std::collections::HashMap::new();
        for i in 0..arr2.len() {
            map.insert(arr2[i], i);
        }
        let mut temp = Vec::new();
        for i in arr1 {
            match map.get(&i) {
                Some(c) => count[map[&i]] += 1,
                None => temp.push(i),
            }
        }
        let mut res = Vec::new();
        for i in 0..arr2.len() {
            for j in 0..count[i] {
                res.push(arr2[i]);
            }
        }
        temp.sort();
        for i in temp {
            res.push(i);
        }
        res
    }
}