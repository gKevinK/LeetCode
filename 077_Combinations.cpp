class Solution {
private:
    void r(vector<vector<int>>& res, vector<int>& c, int p, int a, int n, int k) {
        if (p < k) {
            for (int i = a + 1; i <= n; i++) {
                c[p] = i;
                r(res, c, p + 1, i, n, k);
            }
        } else {
            res.push_back(c);
        }
    }
public:
    vector<vector<int>> combine(int n, int k) {
        vector<vector<int>> res;
        vector<int> c(k, 0);
        r(res, c, 0, 0, n, k);
        return res;
    }
};
