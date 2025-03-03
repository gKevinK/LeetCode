class Solution {
public:
    int numDistinct(string s, string t) {
        if (t.size() == 0) return 0;
        vector<int> v(t.size(), 0);
        for (char & c : s) {
            for (int i = t.size() - 1; i >= 0; i--) {
                if (t[i] == c) {
                    if (i == 0)
                        v[i] = v[i] + 1;
                    else
                        v[i] = v[i] + v[i - 1];
                }
            }
        }
        return v.back();
    }
};