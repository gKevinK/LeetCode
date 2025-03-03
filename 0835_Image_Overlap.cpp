class Solution {
public:
    int largestOverlap(vector<vector<int>>& A, vector<vector<int>>& B) {
        int n = A.size(), r = 0;
        for (int i = -n + 1; i < n; ++i) for (int j = -n + 1; j < n; ++j) {
            int t = 0;
            for (int x = max(0, -i); x < min(n, n - i); ++x)
                for (int y = max(0, -j); y < min(n, n - j); ++y) {
                    t += A[x][y] && B[x + i][y + j];
                }
            r = max(r, t);
        }
        return r;
    }
};
// TODO