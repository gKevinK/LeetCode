class Solution {
public:
    double largestTriangleArea(vector<vector<int>>& points) {
        double r = 0;
        for (auto & i : points) {
            for (auto & j : points) {
                for (auto & k : points) {
                    r = max(r, 0.5 * abs((k[0] - i[0]) * (j[1] - i[1]) - (k[1] - i[1]) * (j[0] - i[0])));
                }
            }
        }
        return r;
    }
};