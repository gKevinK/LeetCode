class Solution {
    int f(vector<int>& sum, vector<vector<int>>& dp, int K, int i, int j) {
        if (i + K > j) return 0;
        if (i + K == j) return sum[j] - sum[i];
        if (dp[i][j] != INT_MAX) return dp[i][j];
        int r = INT_MAX;
        for (int m = i + 1; m < j; m += K - 1)
            r = min(r, f(sum, dp, K, i, m) + f(sum, dp, K, m, j));
        if ((j - i - 1) % (K - 1) == 0)
            r += sum[j] - sum[i];
        dp[i][j] = r;
        return r;
    }
public:
    int mergeStones(vector<int>& stones, int K) {
        int n = stones.size();
        if ((n - 1) % (K - 1) > 0) return -1;
        vector<int> sum(n + 1);
        vector<vector<int>> dp(n + 1, vector<int>(n + 1, INT_MAX));
        sum[0] = 0;
        for (int i = 0; i < n; i++)
            sum[i + 1] = sum[i] + stones[i];
        return f(sum, dp, K, 0, n);
    }
};