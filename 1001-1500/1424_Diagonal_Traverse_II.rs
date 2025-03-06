// impl Solution {
//     pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
//         let mut result = Vec::new();
//         let m = nums.len();
//         let mut v = vec![1; m + 1];
//         v[0] = 0;
//         let mut d = 0;
//         loop {
//             let mut i = std::cmp::min(d + 1, m);
//             if v[i] == 0 {
//                 break;
//             }
//             let mut j = d - i;
//             loop {
//                 let mut l = v[i];
//                 while l > 0 && nums[i - l].len() <= j + l {
//                     l = if v[i - l] > 0 { l + v[i - l] } else { 0 };
//                 }
//                 v[i] = l;
//                 if l == 0 {
//                     break;
//                 }
//                 i -= l;
//                 j += l;
//                 result.push(nums[i][j]);
//             }
//             d += 1;
//         }
//         result
//     }
// }

impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut v = Vec::new();
        for (i, r) in nums.iter().enumerate() {
            for (j, c) in r.iter().enumerate() {
                if v.len() <= i + j {
                    v.push(vec![]);
                }
                v[i + j].push(*c);
            }
        }
        let mut result = Vec::new();
        for (i, r) in v.into_iter().enumerate() {
            for (j, c) in r.into_iter().rev().enumerate() {
                result.push(c);
            }
        }
        result
    }
}