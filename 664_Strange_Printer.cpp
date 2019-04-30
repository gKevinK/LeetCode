class Solution {
    vector<vector<int>> dp;
    string _s;
    
    int f(int l, int r) {
        if (l == r) return 0;
        if (l + 1 == r) return 1;
        if (dp[l][r]) return dp[l][r];
        int res = r - l;
        for (int i = l + 1; i < r; ++i) {
            res = min(res, f(l, i) + f(i, r) - (_s[i] == _s[l]));
        }
        dp[l][r] = res;
        return res;
    }
    
public:
    int strangePrinter(string s) {
        _s = "";
        for (char c : s)
            if (_s.empty() || _s.back() != c)
                _s += c;
        int n = _s.size();
        dp = vector<vector<int>>(n, vector<int>(n + 1, 0));
        return f(0, n);
    }
};