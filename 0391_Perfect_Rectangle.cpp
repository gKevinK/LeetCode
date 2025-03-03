class Solution {
public:
    bool isRectangleCover(vector<vector<int>>& rectangles) {
        vector<int> bound = { INT_MAX, INT_MAX, INT_MIN, INT_MIN };
        int area = 0;
        unordered_set<int> s;
        for (auto & rec : rectangles) {
            bound[0] = min(bound[0], rec[0]);
            bound[1] = min(bound[1], rec[1]);
            bound[2] = max(bound[2], rec[2]);
            bound[3] = max(bound[3], rec[3]);
            area += (rec[2] - rec[0]) * (rec[3] - rec[1]);
        }
        int w = bound[3] - bound[1] + 1;
        for (auto & rec : rectangles) {
            for (auto & pnt : { rec[0] * w + rec[1] - bound[1],
                                rec[0] * w + rec[3] - bound[1],
                                rec[2] * w + rec[1] - bound[1],
                                rec[2] * w + rec[3] - bound[1] }) {
                if (s.find(pnt) != s.end()) s.erase(pnt);
                else s.insert(pnt);
            }
        }
        if (s.size() != 4) return false;
        for (auto & pnt : { bound[0] * w + bound[1] - bound[1],
                            bound[0] * w + bound[3] - bound[1],
                            bound[2] * w + bound[1] - bound[1],
                            bound[2] * w + bound[3] - bound[1] }) {
            if (s.find(pnt) == s.end())
                return false;
        }
        return area == (bound[2] - bound[0]) * (bound[3] - bound[1]);
    }
};