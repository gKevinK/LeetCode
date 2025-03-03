class Solution {
public:
    int swimInWater(vector<vector<int>>& grid) {
        vector<int> dir = { -1, 0, 1, 0, -1 };
        int time = 0, N = grid.size();
        auto visit = vector<vector<bool>>(N, vector<bool>(N, false));
        visit[0][0] = true;
        priority_queue<vector<int>> pq;
        pq.push({ -grid[0][0], 0, 0 });
        while (!pq.empty()) {
            while (time < -pq.top()[0]) ++time;
            int x = pq.top()[1], y = pq.top()[2];
            pq.pop();
            if (x == N - 1 && y == N - 1) return time;
            for (int i = 0; i < 4; ++i) {
                int x2 = x + dir[i], y2 = y + dir[i + 1];
                if (x2 < 0 || x2 >= N || y2 < 0 || y2 >= N) continue;
                if (visit[x2][y2]) continue;
                pq.push({ -grid[x2][y2], x2, y2 });
                visit[x2][y2] = true;
            }
        }
        return -1;
    }
};