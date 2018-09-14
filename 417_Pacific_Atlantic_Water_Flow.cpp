class Solution {
    int m, n;
    
    bool valid(int x, int y) {
        return x >= 0 && x < m && y >= 0 && y < n;
    }
    
    void f(vector<vector<int>>& mat, vector<vector<bool>>& v, queue<pair<int, int>>& q) {
        static vector<vector<int>> dir = { { -1, 0 }, { 0, -1 }, { 1, 0 }, { 0, 1 } };
        while (!q.empty()) {
            int x = q.front().first, y = q.front().second;
            q.pop();
            for (auto & d : dir) {
                int x2 = x + d[0], y2 = y + d[1];
                if (valid(x2, y2) && (!valid(x, y) || mat[x2][y2] >= mat[x][y]) && v[x2][y2] == false) {
                    v[x2][y2] = true;
                    q.push({ x2, y2 });
                }
            }
        }
    }
public:
    vector<pair<int, int>> pacificAtlantic(vector<vector<int>>& matrix) {
        m = matrix.size();
        n = matrix.empty() ? 0 : matrix[0].size();
        vector<vector<bool>> v1(m, vector<bool>(n, false)), v2(m, vector<bool>(n, false));
        queue<pair<int, int>> q1, q2;
        vector<vector<int>> dir = { { -1, 0 }, { 0, -1 }, { 1, 0 }, { 0, 1 } };
        for (int i = 0; i < m; i++) {
            q1.push({ i, 0 });
            v1[i][0] = true;
            q2.push({ i, n - 1 });
            v2[i][n - 1] = true;
        }
        for (int j = 0; j < n; j++) {
            q1.push({ 0, j });
            v1[0][j] = true;
            q2.push({ m - 1, j });
            v2[m - 1][j] = true;
        }
        f(matrix, v1, q1);
        f(matrix, v2, q2);
        vector<pair<int, int>> r;
        for (int i = 0; i < m; i++)
            for (int j = 0; j < n; j++)
                if (v1[i][j] == true && v2[i][j] == true)
                    r.push_back({ i, j });
        return r;
    }
};