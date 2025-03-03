class Solution {
public:
    vector<int> getRow(int rowIndex) {
        vector<int> v;
        v.push_back(1);
        for (int i = 1; i <= rowIndex; ++i) {
            v.push_back(v[i-1] * (long long)(rowIndex + 1 - i) / i);
        }
        return v;
    }
};