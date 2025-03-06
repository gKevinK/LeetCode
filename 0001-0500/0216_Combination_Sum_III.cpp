class Solution {
    vector<vector<int>> f(int k, int r, int n) {
        vector<vector<int>> res;
        if (k == 1 && 0 < n && n <= r) return { { n } };
        if (k == 1) return res;
        for (int i = r; i > n / k; i--) {
            vector<vector<int>> vv = f(k - 1, i - 1, n - i);
            for (vector<int> & v : vv) {
                v.push_back(i);
                res.push_back(v);
            }
        }
        return res;
    }
public:
    vector<vector<int>> combinationSum3(int k, int n) {
        return f(k, 9, n);
    }
};