class Solution {
    bool t(vector<vector<int>>& g, int x, int y) {
        vector<bool> v(16, false);
        for (int di = -1; di <= 1; di++) {
            for (int dj = -1; dj <= 1; dj++) {
                v[g[x + di][y + dj]] = true;
            }
        }
        for (int i = 1; i <= 9; i++)
            if (v[i] == false)
                return false;
        for (int i = -1; i <= 1; i++)
            if (g[x + i][y - 1] + g[x + i][y] + g[x + i][y + 1] != 15)
                return false;
        for (int j = -1; j <= 1; j++)
            if (g[x - 1][y + j] + g[x][y + j] + g[x + 1][y + j] != 15)
                return false;
        if (g[x - 1][y - 1] + g[x][y] + g[x + 1][y + 1] != 15 ||
            g[x + 1][y - 1] + g[x][y] + g[x - 1][y + 1] != 15)
            return false;
        return true;
    }
public:
    int numMagicSquaresInside(vector<vector<int>>& grid) {
        int m = grid.size(), n = grid[0].size(), s = 0;
        for (int i = 1; i < m - 1; i++) {
            for (int j = 1; j < n - 1; j++) {
                if (grid[i][j] == 5 && t(grid, i, j)) {
                    s++;
                }
            }
        }
        return s;
    }
};