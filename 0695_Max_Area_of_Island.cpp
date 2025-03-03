class Solution {
public:
    int maxAreaOfIsland(vector<vector<int>>& grid) {
        static vector<pair<int, int>> v = { { -1, 0 }, { 0, -1 }, { 1, 0 }, { 0, 1 } };
        if (grid.empty()) return 0;
        int r = 0, m = grid.size(), n = grid[0].size();
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                if (grid[i][j] == 1) {
                    int a = 1;
                    queue<pair<int, int>> q;
                    q.push(make_pair(i, j));
                    grid[i][j] = 0;
                    while (!q.empty()) {
                        int x = q.front().first, y = q.front().second;
                        q.pop();
                        for (auto & p : v) {
                            int x1 = x + p.first, y1 = y + p.second;
                            if (x1 >= 0 && x1 < m && y1 >= 0 && y1 < n && grid[x1][y1] == 1) {
                                a++;
                                grid[x1][y1] = 0;
                                q.push(make_pair(x1, y1));
                            }
                        }
                    }
                    r = max(r, a);
                }
            }
        }
        return r;
    }
};