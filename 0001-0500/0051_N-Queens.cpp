class Solution {
    vector<string> m;
    
    void r(vector<vector<string>>& ms, vector<int>& v, int n, int k) {
        if (n == k) {
            for (int i = 0; i < v.size(); i++)
                m[i][v[i]] = 'Q';
            ms.push_back(m);
            for (int i = 0; i < v.size(); i++)
                m[i][v[i]] = '.';
        }
        for (int i = 0; i < n; i++) {
            bool f = false;
            for (int j = 0; j < k; j++) {
                if (i == v[j] || i == v[j] - (k-j) || i == v[j] + (k-j)) {
                    f = true;
                    break;
                }
            }
            if (f) continue;
            v[k] = i;
            r(ms, v, n, k+1);
            v[k] = -1;
        }
    }
public:
    vector<vector<string>> solveNQueens(int n) {
        m = vector<string>(n, string(n, '.'));
        vector<vector<string>> ms;
        vector<int> v(n, -1);
        r(ms, v, n, 0);
        return ms;
    }
};