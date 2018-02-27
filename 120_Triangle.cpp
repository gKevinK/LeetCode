class Solution {
public:
    int minimumTotal(vector<vector<int>>& triangle) {
        vector<int> v(triangle.size() + 1, 0);
        for (int i = triangle.size() - 1; i >= 0; --i) {
            for (int j = 0; j <= i; ++j) {
                v[j] = triangle[i][j] + min(v[j], v[j+1]);
            }
        }
        return v[0];
    }
};