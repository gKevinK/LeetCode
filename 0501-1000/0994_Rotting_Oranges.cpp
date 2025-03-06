class Solution {
public:
    int orangesRotting(vector<vector<int>>& grid) {
        int m = grid.size(), n = grid[0].size(), r = 0;
        deque<vector<int>> q;
        for (int i = 0; i < m; ++i) {
            for (int j = 0; j < n; ++j) {
                if (grid[i][j] == 2)
                    q.push_back({ 0, i, j });
            }
        }
        vector<int> ds = { -1, 0, 1, 0, -1 };
        while (!q.empty()) {
            auto v = q.front();
            q.pop_front();
            for (int i = 0; i < 4; ++i) {
                int x = v[1] + ds[i], y = v[2] + ds[i + 1];
                if (x < 0 || x >= m || y < 0 || y >= n) continue;
                if (grid[x][y] == 1) {
                    grid[x][y] = 2;
                    q.push_back({ v[0] + 1, x, y });
                    r = max(r, v[0] + 1);
                }
            }
        }
        for (int i = 0; i < m; ++i) {
            for (int j = 0; j < n; ++j) {
                if (grid[i][j] == 1) {
                    r = -1;
                    break;
                }
            }
        }
        return r;
    }
};