class Solution {
public:
    int kInversePairs(int n, int k) {
        static int MOD = 1'000'000'007;
        vector<vector<int>> dp(n + 1, vector<int>(k + 1, 0));
        for (int in = 1; in <= n; in++) for (int ik = 0; ik <= k; ik++) {
            if (ik == 0) {
                dp[in][ik] = 1; continue;
            }
            if (in == 1) continue;
            dp[in][ik] = dp[in][ik - 1] + dp[in - 1][ik];
            if (dp[in][ik] > MOD) dp[in][ik] -= MOD;
            if (ik - in >= 0) dp[in][ik] -= dp[in - 1][ik - in];
            if (dp[in][ik] < 0) dp[in][ik] += MOD;
        }
        return dp[n][k];
    }
};