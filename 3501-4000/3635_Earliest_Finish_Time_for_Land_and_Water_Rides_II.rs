impl Solution {
    pub fn earliest_finish_time(land_start_time: Vec<i32>, land_duration: Vec<i32>, water_start_time: Vec<i32>, water_duration: Vec<i32>) -> i32 {
        let n_land = land_start_time.len();
        let n_water = water_start_time.len();
        let mut res = i32::MAX;
        let mut land_end = i32::MAX;
        for i in 0..n_land {
            land_end = land_end.min(land_start_time[i] + land_duration[i]);
        }
        let mut water_end = i32::MAX;
        for i in 0..n_water {
            res = res.min(land_end.max(water_start_time[i]) + water_duration[i]);
            water_end = water_end.min(water_start_time[i] + water_duration[i]);
        }
        for i in 0..n_land {
            res = res.min(water_end.max(land_start_time[i]) + land_duration[i]);
        }
        res
    }
}