class Solution {
public:
    int findMinArrowShots(vector<pair<int, int>>& points) {
        int end = INT_MIN, res = 0;
        sort(points.begin(), points.end(), [](const auto & a, const auto & b) { return a.second < b.second; });
        for (auto & p : points) {
            if (end == INT_MIN || p.first > end) {
                res++;
                end = p.second;
            }
        }
        return res;
    }
};