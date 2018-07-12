class Solution {
public:
    int integerBreak(int n) {
        static vector<int> v = { 0, 1 };
        while (v.size() <= n) {
            int c = v.size(), r = 0;
            for (int i = 1; i < c; i++)
                r = max(r, max(i, v[i]) * (c - i));
            v.push_back(r);
        }
        return v[n];
    }
};