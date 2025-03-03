class Solution {
public:
    int surfaceArea(vector<vector<int>>& grid) {
        int m = grid.size(), n = grid.empty() ? 0 : grid[0].size(), area = 0;
        for (int i = 0; i < m; i++)
            for (int j = 0; j < n; j++) {
                if (grid[i][j] == 0) continue;
                area += grid[i][j] * 4 + 2;
                if (i > 0) area -= min(grid[i][j], grid[i - 1][j]) * 2;
                if (j > 0) area -= min(grid[i][j], grid[i][j - 1]) * 2;
            }
        return area;
    }
};