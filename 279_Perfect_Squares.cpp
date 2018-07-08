class Solution {
public:
    int numSquares(int n) {
        static vector<int> sqrs, v = { 0 };
        while (sqrs.size() * sqrs.size() <= n)
            sqrs.push_back((sqrs.size() + 1) * (sqrs.size() + 1));
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