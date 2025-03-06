impl Solution {
    pub fn filter_restaurants(restaurants: Vec<Vec<i32>>, vegan_friendly: i32, max_price: i32, max_distance: i32) -> Vec<i32> {
        let mut v = Vec::new();
        for r in restaurants {
            if vegan_friendly == 1 && r[2] == 0 {
                continue;
            }
            if r[3] > max_price {
                continue;
            }
            if r[4] > max_distance {
                continue;
            }
            v.push((r[1], r[0]));
        }
        v.sort();
        v.into_iter().rev().map(|r| r.1).collect()
    }
}