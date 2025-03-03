class Solution {
public:
    int minFallingPathSum(vector<vector<int>>& A) {
        int m = A.size(), n = A[0].size(), r = INT_MAX;
        vector<int> v1(n, 0), v2(n);
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++)
                v2[j] = v1[j];
            for (int j = 1; j < n; j++)
                v2[j] = min(v2[j], v1[j - 1]);
            for (int j = 0; j < n - 1; j++)
                v2[j] = min(v2[j], v1[j + 1]);
            for (int j = 0; j < n; j++)
                v1[j] = v2[j] + A[i][j];
        }
        for (int j = 0; j < n; j++)
            r = min(r, v1[j]);
        return r;
    }
};