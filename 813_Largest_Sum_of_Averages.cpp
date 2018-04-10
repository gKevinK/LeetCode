class Solution {
    double f(int n, int k, vector<int>& A, vector<vector<double>>& dp) {
        if (dp[n][k] > 0) return dp[n][k];
        double m = 0, s = 0;
        for (int i = n - 1; k - 1 <= i; i--) {
            s += A[i];
            if (k - 1 > 0)
                m = max(m, f(i, k - 1, A, dp) + s / (n - i));
            else if (i == 0 && k == 1)
                m = max(m, s / n);
        }
        dp[n][k] = m;
        return m;
    }
public:
    double largestSumOfAverages(vector<int>& A, int K) {
        vector<vector<double>> dp(A.size() + 1, vector<double>(K + 1, 0.0));
        return f(A.size(), K, A, dp);
    }
};