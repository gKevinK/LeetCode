class Solution {
public:
    int leastBricks(vector<vector<int>>& wall) {
        int r = 0, n = wall.size(), w = accumulate(wall[0].begin(), wall[0].end(), 0);
        unordered_map<int, int> m;
        for (auto & row : wall) {
            int l = 0;
            for (int & i : row) {
                l += i;
                m[l]++;
            }
        }
        m.erase(w);
        for (auto & p : m)
            r = max(r, p.second);
        return n - r;
    }
};