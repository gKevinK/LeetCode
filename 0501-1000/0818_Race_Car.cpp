class Solution {
    int f(int t, vector<int>& dp) {
        if (dp[t]) return dp[t];
        int n = 1;
        for (; 1 << n <= t + 1; ++n) {
            if (1 << n == t + 1) {
                dp[t] = n;
                return n;
            }
        }
        int r = f((1 << n) - 1 - t, dp) + n + 1;
        for (int i = 0; i < n - 1; ++i) {
            r = min(r, f(t - (1 << (n - 1)) + (1 << i), dp) + n + i + 1);
        }
        dp[t] = r;
        return r;
    }
public:
    int racecar(int target) {
        vector<int> dp(target + 1, 0);
        return f(target, dp);
    }
};