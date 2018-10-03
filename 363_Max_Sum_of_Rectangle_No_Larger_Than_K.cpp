class Solution {
public:
    int maxSumSubmatrix(vector<vector<int>>& matrix, int k) {
        int m = matrix.size(), n = matrix[0].size(), res = INT_MIN;
        vector<vector<int>> s(m + 1, vector<int>(n + 1, 0));
        for (int i = 1; i <= m; i++)
            for (int j = 1; j <= n; j++) {
                s[i][j] = matrix[i - 1][j - 1] + s[i - 1][j] + s[i][j - 1] - s[i - 1][j - 1];
            }
        for (int i = 0; i < m; i++)
            for (int j = 0; j < n; j++)
                for (int p = 1; p <= m - i; p++)
                    for (int q = 1; q <= n - j; q++){
                        int sum = s[i + p][j + q] - s[i][j + q] - s[i + p][j] + s[i][j];
                        if (sum <= k && sum > res)
                            res = sum;
                    }
        return res;
    }
};