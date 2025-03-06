class Solution {
public:
    vector<vector<int>> kClosest(vector<vector<int>>& points, int K) {
        nth_element(points.begin(), points.begin() + K - 1, points.end(),
            [&points](auto & p1, auto & p2) {
                return p1[0] * p1[0] + p1[1] * p1[1] < p2[0] * p2[0] + p2[1] * p2[1];
            });
        return vector<vector<int>>(points.begin(), points.begin() + K);
    }
};