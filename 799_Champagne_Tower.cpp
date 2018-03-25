class Solution {
public:
    double champagneTower(int poured, int query_row, int query_glass) {
        vector<double> v(query_row + 1, 0);
        v[0] = poured;
        for (int i = 0; i < query_row; i++) {
            for (int j = min(i, query_glass); j >= max(0, query_glass - query_row + i); j--) {
                if (j + 1 < v.size())
                    v[j + 1] += max(v[j] - 1.0, 0.0) / 2.0;
                v[j] = max(v[j] - 1.0, 0.0) / 2.0;
            }
        }
        return min(v[query_glass], 1.0);
    }
};