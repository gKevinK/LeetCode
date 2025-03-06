class Solution {
public:
    int numberOfBoomerangs(vector<pair<int, int>>& points) {
        int n = points.size(), res = 0;
        vector<vector<double>> d(n, vector<double>(n, 0));
        for (int i = 0; i < n; i++)
            for (int j = i; j < n; j++)
                d[i][j] = d[j][i] = sqrt(pow(points[i].first - points[j].first, 2.0)
                                         + pow(points[i].second - points[j].second, 2.0));
        for (auto v : d) {
            sort(v.begin(), v.end());
            for (int i = 0, j = 0; i < n; i = j) {
                j = i + 1;
                while (j < n && v[j] == v[i]) j++;
                if (j - i > 1) res += (j - i) * (j - i - 1);
            }
        }
        return res;
    }
};