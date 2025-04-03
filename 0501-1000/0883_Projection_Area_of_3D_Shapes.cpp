class Solution {
public:
    int projectionArea(vector<vector<int>>& grid) {
        int m = grid.size(), n = grid[0].size(), r = 0;
        vector<int> vf(m, 0), vs(n, 0);
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                r += grid[i][j] > 0;
                vf[i] = max(vf[i], grid[i][j]);
                vs[j] = max(vs[j], grid[i][j]);
            }
        }
        for (int & i : vf) r += i;
        for (int & i : vs) r += i;
        return r;
    }
};