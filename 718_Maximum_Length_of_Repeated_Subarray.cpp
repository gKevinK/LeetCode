class Solution {
public:
    int findLength(vector<int>& A, vector<int>& B) {
        int la = A.size(), lb = B.size(), m = 0;
        vector<int> dp(lb + 1, 0);
        for (int i = 0; i < la; ++i) {
            for (int j = lb - 1; j >= 0; --j) {
                if (A[i] == B[j]) {
                    dp[j + 1] = dp[j] + 1;
                    m = max(m, dp[j + 1]);
                } else
                    dp[j + 1] = 0;
            }
        }
        return m;
    }
};