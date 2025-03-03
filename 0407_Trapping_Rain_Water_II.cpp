class Solution {
public:
    int trapRainWater(vector<vector<int>>& heightMap) {
        if (heightMap.size() < 3 || heightMap[0].size() < 3) return 0;
        int m = heightMap.size(), n = heightMap[0].size(), volumn = 0;
        vector<vector<int>> water(m, vector<int>(n, 20001));
        priority_queue<pair<int, int>, vector<pair<int, int>>, greater<pair<int, int>>> pq;
        for (int i = 0; i < m; i++) {
            water[i][0] = heightMap[i][0];
            water[i][n-1] = heightMap[i][n-1];
            pq.push({ water[i][0], i * 1000 });
            pq.push({ water[i][n-1], i * 1000 + n - 1 });
        }
        for (int j = 1; j < n - 1; j++) {
            water[0][j] = heightMap[0][j];
            water[m-1][j] = heightMap[m-1][j];
            pq.push({ water[0][j], j });
            pq.push({ water[m-1][j], (m-1) * 1000 + j });
        }
        static vector<vector<int>> dir = { { -1, 0 }, { 0, -1 }, { 1, 0 }, { 0, 1 } };
        while (!pq.empty()) {
            int w = pq.top().first, x = pq.top().second / 1000, y = pq.top().second % 1000;
            pq.pop();
            for (auto & d : dir) {
                int x2 = x + d[0], y2 = y + d[1];
                if (x2 < 0 || x2 >= m || y2 < 0 || y2 >= n) continue;
                if (water[x2][y2] <= w || water[x2][y2] == heightMap[x2][y2]) continue;
                water[x2][y2] = max(heightMap[x2][y2], w);
                pq.push({ water[x2][y2], x2 * 1000 + y2 });
            }
        }
        for (int i = 0; i < m; i++)
            for (int j = 0; j < n; j++)
                volumn += water[i][j] - heightMap[i][j];
        return volumn;
    }
};