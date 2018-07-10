class Solution {
public:
    vector<vector<int>> transpose(vector<vector<int>>& A) {
        int m = A.size(), n = A[0].size();
        vector<vector<int>> r(n, vector<int>(m, 0));
        for (int i = 0; i < m; i++)
            for (int j = 0; j < n; j++)
                r[j][i] = A[i][j];
        return r;
    }
};