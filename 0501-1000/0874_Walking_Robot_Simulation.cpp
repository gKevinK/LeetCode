class Solution {
public:
    int robotSim(vector<int>& commands, vector<vector<int>>& obstacles) {
        set<pair<int, int>> s;
        for (auto & o : obstacles)
            s.insert({ o[0], o[1] });
        int d = 0, x = 0, y = 0, res = 0;
        vector<pair<int, int>> step = { { 0, 1 }, { 1, 0 }, { 0, -1 }, { -1, 0 } };
        for (int & c : commands) {
            if (c < 0) {
                if (c == -2) d += 3;
                else d++;
                d %= 4;
                continue;
            }
            while (c-- > 0 && s.find({ x + step[d].first, y + step[d].second }) == s.end()) {
                x += step[d].first;
                y += step[d].second;
            }
            res = max(res, x * x + y * y);
        }
        return res;
    }
};