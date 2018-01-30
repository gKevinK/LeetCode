class Solution {
    void r(int & ms, vector<int>& v, int n, int k) {
        if (n == k) {
            ms++;
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
    int totalNQueens(int n) {
        int ms = 0;
        vector<int> v(n, -1);
        r(ms, v, n, 0);
        return ms;
    }
};