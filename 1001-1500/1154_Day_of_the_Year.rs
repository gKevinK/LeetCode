impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        let year: i32 = date[0..4].to_string().parse().unwrap();
        let month: i32 = date[5..7].to_string().parse().unwrap();
        let day: i32 = date[8..10].to_string().parse().unwrap();
        let a = (year % 4 == 0) && (year % 100 != 0) || (year % 400 == 0);
        let arr = [ 31, 28 + a as i32, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31 ];
        let mut res = day;
        for i in 1..month {
            res += arr[(i - 1) as usize];
        }
        res
    }
}