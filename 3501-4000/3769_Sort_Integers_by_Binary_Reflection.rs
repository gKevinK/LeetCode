impl Solution {
    pub fn sort_by_reflection(nums: Vec<i32>) -> Vec<i32> {
        let key = |mut num| {
            let mut refl = 0;
            while num > 0 {
                refl = (refl << 1) + (num & 1);
                num >>= 1;
            }
            refl
        };
        let mut arr = nums.iter().map(|&num| (key(num), num)).collect::<Vec<_>>();
        arr.sort_unstable();
        arr.iter().map(|&(_, num)| num).collect()
    }
}