impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        let n = books.len();
        let mut dp = vec![std::i32::MAX; n + 1];
        dp[0] = 0;
        for i in 0..n {
            let mut width = 0;
            let mut max_h = 0;
            for j in i..n {
                width += books[j][0];
                max_h = std::cmp::max(max_h, books[j][1]);
                if width > shelf_width {
                    break;
                }
                dp[j + 1] = std::cmp::min(dp[j + 1], dp[i] + max_h);
            }
        }
        dp[n]
    }
}