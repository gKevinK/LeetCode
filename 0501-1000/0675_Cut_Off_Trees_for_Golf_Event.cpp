// class Solution {
//     int distance(vector<vector<int>>& f, vector<vector<bool>>& v, pair<int, int> s, pair<int, int> t) {
//         static vector<pair<int, int>> dir = { { -1, 0 }, { 1, 0 }, { 0, -1 }, { 0, 1 } };
//         for (auto && r : v)
//             for (auto && c : r)
//                 c = false;
//         v[s.first][s.second] = true;
//         queue<pair<int, int>> q;
//         q.push(s);
//         for (int dist = 1; !q.empty(); ++dist) {
//             for (int i = q.size(); i; --i) {
//                 auto p = q.front();
//                 q.pop();
//                 for (auto & d : dir) {
//                     int x = p.first + d.first, y = p.second + d.second;
//                     if (x < 0 || x >= f.size() || y < 0 || y >= f[0].size()) continue;
//                     if (f[x][y] == 0 || v[x][y]) continue;
//                     if (t.first == x && t.second == y) return dist;
//                     q.push({ x, y });
//                     v[x][y] = true;
//                 }
//             }
//         }
//         return -1;
//     }
// public:
//     int cutOffTree(vector<vector<int>>& forest) {
//         vector<pair<int, pair<int, int>>> v;
//         int m = forest.size(), n = forest[0].size(), r = 0;
//         for (int i = 0; i < m; ++i)
//             for (int j = 0; j < n; ++j)
//                 if (forest[i][j] > 1)
//                     v.push_back({ forest[i][j], { i, j } });
//         sort(v.begin(), v.end());
        
//         vector<vector<bool>> visited(m, vector<bool>(n));
//         pair<int, int> src = { 0, 0 };
//         for (int i = v[0].second == src ? 1 : 0; i < v.size(); ++i) {
//             auto tar = v[i].second;
//             int d = distance(forest, visited, src, tar);
//             if (d == -1) return -1;
//             r += d;
//             src = tar;
//         }        
//         return r;
//     }
// };

class Solution {
    int distance(vector<vector<int>>& f, pair<int, int> s, pair<int, int> t) {
        static vector<pair<int, int>> dir = { { -1, 0 }, { 1, 0 }, { 0, -1 }, { 0, 1 } };
        int m = f.size(), n = f[0].size(), base = abs(s.first - t.first) + abs(s.second - t.second);
        vector<bool> v(m * n, false);
        deque<pair<int, pair<int, int>>> q;
        q.push_back({ 0, s });
        while (!q.empty()) {
            int det = q.front().first;
            auto p = q.front().second;
            q.pop_front();
            if (v[p.first * n + p.second]) continue;
            v[p.first * n + p.second] = true;
            for (auto & d : dir) {
                int x = p.first + d.first, y = p.second + d.second;
                if (x < 0 || x >= m || y < 0 || y >= n) continue;
                if (f[x][y] == 0) continue;
                if (t.first == x && t.second == y) return base + 2 * det;
                bool close = d.first * (t.first - p.first) > 0 || d.second * (t.second - p.second) > 0;
                if (close) q.push_front({ det, { x, y } });
                else q.push_back({ det + 1, { x, y } });
            }
        }
        return -1;
    }
public:
    int cutOffTree(vector<vector<int>>& forest) {
        vector<pair<int, pair<int, int>>> v;
        int m = forest.size(), n = forest[0].size(), r = 0;
        for (int i = 0; i < m; ++i)
            for (int j = 0; j < n; ++j)
                if (forest[i][j] > 1)
                    v.push_back({ forest[i][j], { i, j } });
        sort(v.begin(), v.end());
        
        pair<int, int> src = { 0, 0 };
        for (int i = v[0].second == src ? 1 : 0; i < v.size(); ++i) {
            auto tar = v[i].second;
            int d = distance(forest, src, tar);
            if (d == -1) return -1;
            r += d;
            src = tar;
        }        
        return r;
    }
};