class Solution {
public:
    int matrixScore(vector<vector<int>>& A) {
        int m = A.size(), n = A[0].size();
        int s = (1 << (n - 1)) * m;
        for (int j = 1; j < n; j++) {
            int x = 0;
            for (int i = 0; i < m; i++)
                if ((A[i][0] + A[i][j]) % 2 == 0)
                    x++;
            s += max(x, m - x) * (1 << (n - j - 1));
        }
        return s;
    }
};