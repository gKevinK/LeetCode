class Solution {
    int distance(vector<vector<int>>& f, vector<vector<bool>>& v, pair<int, int> s, pair<int, int> t) {
        static vector<pair<int, int>> dir = { { -1, 0 }, { 1, 0 }, { 0, -1 }, { 0, 1 } };
        for (auto && r : v)
            for (auto && c : r)
                c = false;
        v[s.first][s.second] = true;
        queue<pair<int, int>> q;
        q.push(s);
        for (int dist = 1; !q.empty(); ++dist) {
            for (int i = q.size(); i; --i) {
                auto p = q.front();
                q.pop();
                for (auto & d : dir) {
                    int x = p.first + d.first, y = p.second + d.second;
                    if (x < 0 || x >= f.size() || y < 0 || y >= f[0].size()) continue;
                    if (f[x][y] == 0 || v[x][y]) continue;
                    if (t.first == x && t.second == y) return dist;
                    q.push({ x, y });
                    v[x][y] = true;
                }
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
        
        vector<vector<bool>> visited(m, vector<bool>(n));
        pair<int, int> src = { 0, 0 };
        for (int i = v[0].second == src ? 1 : 0; i < v.size(); ++i) {
            auto tar = v[i].second;
            int d = distance(forest, visited, src, tar);
            if (d == -1) return -1;
            r += d;
            src = tar;
        }        
        return r;
    }
};