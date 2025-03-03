class Solution {
public:
    int largestIsland(vector<vector<int>>& grid) {
        int n = grid.size(), s = 0;
        vector<vector<int>> g(n, vector<int>(n, 0));
        vector<vector<int>> ds = { { -1, 0 }, { 0, -1 }, { 1, 0 }, { 0, 1 } };
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                if (grid[i][j] == 1 && g[i][j] != -1) {
                    queue<pair<int, int>> q;
                    set<pair<int, int>> ve;
                    int a = 1;
                    g[i][j] = -1;
                    q.push(make_pair(i, j));
                    while (!q.empty()) {
                        int x = q.front().first, y = q.front().second;
                        q.pop();
                        for (auto & d : ds) {
                            int x1 = x + d[0], y1 = y + d[1];
                            if (x1 < 0 || x1 >= n || y1 < 0 || y1 >= n)
                                continue;
                            if (grid[x1][y1] == 1 && g[x1][y1] == -1)
                                continue;
                            if (grid[x1][y1] == 1) {
                                a++;
                                g[x1][y1] = -1;
                                q.push(make_pair(x1, y1));
                            } else {
                                ve.insert(make_pair(x1, y1));
                            }
                        }
                    }
                    s = max(s, a);
                    for (auto & p : ve) {
                        g[p.first][p.second] += a;
                        s = max(s, g[p.first][p.second] + 1);
                    }
                }
            }
        }
        return max(1, s);
    }
};