impl Solution {
    pub fn find_common_response(responses: Vec<Vec<String>>) -> String {
        let mut resp: Vec<_> = responses.into_iter()
            .flat_map(|mut x| {
                x.sort();
                x.dedup();
                x
            }).collect();
        resp.sort();
        let mut count = 0;
        let mut r = String::new();
        let mut i = 0;
        while i < resp.len() {
            let mut j = i + 1;
            while j < resp.len() && resp[i] == resp[j] {
                j += 1;
            }
            if j - i > count || (j - i == count && resp[i] < r) {
                r = resp[i].clone();
                count = j - i;
            }
            i = j;
        }
        r
    }
}