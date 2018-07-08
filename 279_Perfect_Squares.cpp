class Solution {
public:
    int numSquares(int n) {
        vector<int> sqrs, v = { 0 };
        for (int i = 1; i * i <= n; i++)
            sqrs.push_back(i * i);
        while (v.size() <= n) {
            int r = INT_MAX, i = v.size();
            for (int & sqr : sqrs) {
                if (i - sqr < 0) break;
                r = min(r, v[i - sqr] + 1);
            }
            v.push_back(r);
        }
        return v[n];
    }
};