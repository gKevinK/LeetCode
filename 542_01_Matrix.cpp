class Solution {
public:
    vector<vector<int>> updateMatrix(vector<vector<int>>& matrix) {
        int m = matrix.size(), n = matrix[0].size();
        vector<vector<int>> r(m, vector<int>(n, -1));
        queue<pair<int, int>> q;
        static vector<pair<int, int>> d = { { -1, 0 }, { 1, 0 }, { 0, -1 }, { 0, 1 } };
        for (int i = 0; i < m; i++)
            for (int j = 0; j < n; j++)
                if (matrix[i][j] == 0) {
                    q.push({ i, j });
                    r[i][j] = 0;
                }
        while (!q.empty()) {
            int i = q.front().first, j = q.front().second;
            q.pop();
            for (auto & p : d) {
                int x = i + p.first, y = j + p.second;
                if (x >= 0 && x < m && y >= 0 && y < n && r[x][y] == -1) {
                    q.push({ x, y });
                    r[x][y] = r[i][j] + 1;
                }
            }
        }
        return r;
    }
};