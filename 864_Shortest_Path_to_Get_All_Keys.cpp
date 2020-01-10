class Solution {
public:
    int shortestPathAllKeys(vector<string>& grid) {
        int dir[] = { -1, 0, 1, 0, -1 };
        int M = grid.size(), N = grid[0].size(), K = 0;
        queue<pair<int, int>> q;
        for (int i = 0; i < M; ++i) for (int j = 0; j < N; ++j) {
            if (grid[i][j] == '@') q.push({ i * N + j, 0 });
            if ('a' <= grid[i][j] && grid[i][j] <= 'f') ++K;
        }
        vector<vector<bool>> dp(M * N, vector<bool>(1 << K, false));
        dp[q.front().first][0] = true;
        for (int move = 0; !q.empty(); ++move) {
            for (int _a = q.size(); _a; --_a) {
                int x = q.front().first / N, y = q.front().first % N, k = q.front().second;
                q.pop();
                if (k == (1 << K) - 1) return move;
                for (int i = 0; i < 4; ++i) {
                    int x2 = x + dir[i], y2 = y + dir[i + 1], k2 = k;
                    if (x2 < 0 || x2 >= M || y2 < 0 || y2 >= N) continue;
                    char c = grid[x2][y2];
                    if (c == '#') continue;
                    if ('a' <= c && c <= 'f')
                        k2 |= 1 << (c - 'a');
                    if ('A' <= c && c <= 'F')
                        if (!(k2 & (1 << (c - 'A'))))
                            continue;
                    if (dp[x2 * N + y2][k2]) continue;
                    dp[x2 * N + y2][k2] = true;
                    q.push({ x2 * N + y2, k2 });
                }
            }
        }
        return -1;
    }
};