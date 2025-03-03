class Solution {
public:
    vector<vector<int>> generate(int numRows) {
        vector<vector<int>> r;
        vector<int> v;
        for (int i = 0; i < numRows; ++i) {
            v.clear();
            v.resize(i + 1);
            v[0] = v[i] = 1;
            for (int j = 1; j < i; ++j) {
                v[j] = r[i-1][j-1] + r[i-1][j];
            }
            r.push_back(v);
        }
        return r;
    }
};