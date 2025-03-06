class Solution {
public:
    int numTrees(int n) {
        vector<int> v = { 0 };
        v.resize(n + 1);
        v[0] = 1; v[1] = 1;
        for (int i = 2; i < n + 1; ++ i) {
            for (int j = 0; j < i; ++j) {
                v[i] += v[j] * v[i-1-j];
            }
        }
        return v[n];
    }
};