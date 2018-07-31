class Solution {
public:
    vector<int> findAnagrams(string s, string p) {
        int l = s.size(), n = p.size();
        vector<int> v1(26, 0), v2(26, 0), r;
        if (l < n)
            return r;
        for (int i = 0; i < n; i++) {
            v1[p[i] - 'a']++;
            v2[s[i] - 'a']++;
        }
        if (v1 == v2)
            r.push_back(0);
        for (int i = n; i < l; i++) {
            v2[s[i] - 'a']++;
            v2[s[i - n] - 'a']--;
            if (v1 == v2)
                r.push_back(i - n + 1);
        }
        return r;
    }
};