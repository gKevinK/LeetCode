impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        let mut d = 5;
        for i in 1971..year {
            if ((i % 4 == 0) && (i % 100 != 0)) || (i % 400 == 0) {
                d += 2;
            } else {
                d += 1;
            }
        }
        let mut v = [ 0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31 ];
        if ((year % 4 == 0) && (year % 100 != 0)) || (year % 400 == 0) {
            v[2] += 1;
        }
        for i in 1..month {
            d += v[i as usize];
        }
        d += day - 1;
        let o = [ "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday" ];
        o[(d % 7) as usize].to_string()
    }
}