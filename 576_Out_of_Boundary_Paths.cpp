class Solution {
public:
    int findPaths(int m, int n, int N, int i, int j) {
        int DIV = 1000000007, res = 0;
        vector<pair<int, int>> dir = { { -1, 0 }, { 1, 0 }, { 0, -1 }, { 0, 1 } };
        vector<vector<int>> dp(m, vector<int>(n, 0)), dp2(m, vector<int>(n, 0));
        dp[i][j] = 1;
        for (int step = 0; step < N; step++) {
            for (int x = 0; x < m; x++) for (int y = 0; y < n; y++) {
                for (auto d : dir) {
                    int nx = x + d.first, ny = y + d.second;
                    if (nx < 0 || ny < 0 || nx == m || ny == n)
                        res = (res + dp[x][y]) % DIV;
                    else
                        dp2[nx][ny] = (dp2[nx][ny] + dp[x][y]) % DIV;
                }
                dp[x][y] = 0;
            }
            dp.swap(dp2);
        }
        return res;
    }
};