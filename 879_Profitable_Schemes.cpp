class Solution {
public:
    int profitableSchemes(int G, int P, vector<int>& group, vector<int>& profit) {
        vector<vector<int>> dp(P + 1, vector<int>(G + 1, 0));
        int n = group.size(), r = 0, mod = 1e9 + 7;
        dp[0][0] = 1;
        for (int k = 0; k < n; k++) {
            // cout << "=" << endl;
            int g = group[k], p = profit[k];
            for (int i = P; i >= 0; i--) {
                for (int j = G - g; j >= 0; j--) {
                    dp[min(P, i + p)][j + g] += dp[i][j];
                    // if (dp[i][j]) cout << min(P, i + p) << " " << j + g << " " << dp[min(P, i + p)][j + g] << endl;
                    dp[min(P, i + p)][j + g] %= mod;
                }
            }
        }
        for (int & i : dp[P])
            r = (r + i) % mod;
        return r;
    }
};