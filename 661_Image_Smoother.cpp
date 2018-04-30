class Solution {
public:
    vector<vector<int>> imageSmoother(vector<vector<int>>& M) {
        int m = M.size(), n = M[0].size();
        vector<vector<int>> p(m, vector<int>(n, 0));
        vector<vector<int>> t = { { -1, -1 }, { -1, 0 }, { -1, 1 }, { 0, 1 }, { 1, 1 }, { 1, 0 }, { 1, -1 }, { 0, -1 }, { 0, 0 } };
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                int c = 0;
                for (auto & d : t) {
                    int x = i + d[0], y = j + d[1];
                    if (0 <= x && x < m && 0 <= y && y < n) {
                        c++;
                        p[i][j] += M[x][y];
                    }
                }
                p[i][j] /= c;
            }
        }
        return p;
    }
};