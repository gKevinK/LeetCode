class Solution {
public:
    int minPathSum(vector<vector<int>>& grid) {
        if (grid.empty() || grid[0].empty()) return 0;
        int m = grid.size(), n = grid[0].size();
        vector<int> v(n, INT_MAX);
        v[0] = 0;
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                v[j] = min(v[j], j == 0 ? INT_MAX : v[j - 1]) + grid[i][j];
            }
        }
        return v.back();
    }
};