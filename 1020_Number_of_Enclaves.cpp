class Solution {
    vector<vector<int>> ds = { { 1, 0 }, { 0, 1 }, { -1, 0 }, { 0, -1 } };
public:
    int numEnclaves(vector<vector<int>>& A) {
        queue<pair<int, int>> q;
        int m = A.size(), n = A[0].size();
        for (int i = 0; i < m; ++i) {
            if (A[i][0] == 1) q.push({ i, 0 });
            if (A[i][n - 1] == 1) q.push({ i,  n - 1 });
        }
        for (int j = 0; j < n; ++j) {
            if (A[0][j] == 1) q.push({ 0, j });
            if (A[m - 1][j] == 1) q.push({ m - 1, j });
        }
        while (!q.empty()) {
            auto p = q.front();
            q.pop();
            if (A[p.first][p.second] == 0) continue;
            A[p.first][p.second] = 0;
            for (auto & d : ds) {
                int x = p.first + d[0], y = p.second + d[1];
                if (x < 0 || x >= m || y < 0 || y >= n || A[x][y] == 0)
                    continue;
                q.push({ x, y });
            }
        }
        int c = 0;
        for (int i = 0; i < m; ++i) for (int j = 0; j < n; ++j) {
            c += A[i][j];
        }
        return c;
    }
};