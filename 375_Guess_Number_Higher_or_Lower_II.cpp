class Solution {
public:
    int getMoneyAmount(int n) {
        vector<vector<int>> dp(n + 1, vector<int>(n + 1, 0));
        for (int j = 1; j <= n; j++)
            for (int i = j - 1; i; i--) {
                if (i == j - 1)
                    dp[i][j] = i;
                else {
                    int r = INT_MAX;
                    for (int k = i; k < j; k++)
                        r = min(r, k + max(dp[i][k - 1], dp[k + 1][j]));
                    dp[i][j] = r;
                }
            }
        return dp[1][n];
    }
};