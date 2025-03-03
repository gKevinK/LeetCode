class Solution {
public:
    string smallestGoodBase(string n) {
        long long x = stoll(n);
        for (int i = 60; i >= 3; i--) {
            long long lo = pow(2, log2(x / 2) / (i - 1)), hi = pow(2, log2(x) / (i - 1)), mi;
            lo = max(lo, 2ll);
            while (lo <= hi) {
                mi = (hi - lo) / 2 + lo;
                long long r = 1;
                for (int d = i - 1; d; d--)
                    r = r * mi + 1;
                if (r == x)
                    return to_string(mi);
                else if (r < x)
                    lo = mi + 1;
                else
                    hi = mi - 1;
            }
        }
        return to_string(x - 1);
    }
};