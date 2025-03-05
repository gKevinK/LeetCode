impl Solution {
    pub fn convert_date_to_binary(date: String) -> String {
        let mut v: Vec<_> = date.split('-').collect();
        let w: Vec<_> = v.into_iter().map(|x| format!("{:b}", x.parse::<i32>().unwrap())).collect();
        w.join("-")
    }
}