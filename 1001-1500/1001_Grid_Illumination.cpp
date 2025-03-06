class Solution {
public:
    vector<int> gridIllumination(int N, vector<vector<int>>& lamps, vector<vector<int>>& queries) {
        vector<unordered_map<int, int>> m(4);
        set<pair<int, int>> s;
        vector<int> res;
        for (auto & l : lamps) {
            s.insert({ l[0], l[1] });
            m[0][l[0]]++;
            m[1][l[1]]++;
            m[2][l[0] + l[1]]++;
            m[3][l[0] - l[1]]++;
        }
        for (auto & q : queries) {
            res.push_back( m[0].find(q[0]) != m[0].end()
                        || m[1].find(q[1]) != m[1].end()
                        || m[2].find(q[0] + q[1]) != m[2].end()
                        || m[3].find(q[0] - q[1]) != m[3].end());
            for (int x = q[0] - 1; x <= q[0] + 1; x++)
                for (int y = q[1] - 1; y <= q[1] + 1; y++)
                    if (s.find({ x, y }) != s.end()) {
                        m[0][x]--;
                        m[1][y]--;
                        m[2][x + y]--;
                        m[3][x - y]--;
                        if (m[0][x] == 0) m[0].erase(x);
                        if (m[1][y] == 0) m[1].erase(y);
                        if (m[2][x + y] == 0) m[2].erase(x + y);
                        if (m[3][x - y] == 0) m[3].erase(x - y);
                        s.erase({ x, y });
                    }
        }
        return res;
    }
};