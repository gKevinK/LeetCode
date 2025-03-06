class Solution {
public:
    int minimumDeleteSum(string s1, string s2) {
        int m = s1.size(), n = s2.size();
        vector<int> dp((m + 1) * (n + 1), -1);
        dp[0] = 0;
        for (int i = 0; i < m; ++i)
            dp[(i + 1) * (n + 1)] = dp[i * (n + 1)] + s1[i];
        for (int j = 0; j < n; ++j)
            dp[j + 1] = dp[j] + s2[j];
        for (int i = 1; i <= m; ++i)
            for (int j = 1; j <= n; ++j) {
                if (s1[i - 1] == s2[j - 1])
                    dp[i * (n + 1) + j] = dp[(i - 1) * (n + 1) + j - 1];
                else
                    dp[i * (n + 1) + j] = min(
                        dp[(i - 1) * (n + 1) + j] + s1[i - 1],
                        dp[i * (n + 1) + j - 1] + s2[j - 1]);
            }
        return dp[m * (n + 1) + n];
    }
};