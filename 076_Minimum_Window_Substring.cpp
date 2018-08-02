class Solution {
public:
    string minWindow(string s, string t) {
        vector<int> v(128, 0);
        for (char & c : t)
            v[c]++;
        int i = 0, j = 0, l = 0, r = 0, c = 0;
        for (int & n : v)
            if (n > 0)
                c++;
        while (j < s.size()) {
            if (--v[s[j++]] == 0) c--;
            while (v[s[i]] < 0)
                v[s[i++]]++;
            if (c == 0 && (r == 0 || r - l > j - i)) {
                l = i;
                r = j;
            }
        }
        return s.substr(l, r - l);
    }
};