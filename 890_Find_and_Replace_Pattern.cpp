class Solution {
    bool test(const string & w, const string & p) {
        vector<char> m(26, -1), r(26, -1);
        for (int i = 0; i < p.size(); i++) {
            if (m[p[i] - 'a'] == w[i])
                continue;
            else if (m[p[i] - 'a'] != -1 && m[p[i] - 'a'] != w[i])
                return false;
            else if (r[w[i] - 'a'] == -1) {
                m[p[i] - 'a'] = w[i];
                r[w[i] - 'a'] = p[i];
            } else
                return false;
        }
        return true;
    }
public:
    vector<string> findAndReplacePattern(vector<string>& words, string pattern) {
        vector<string> res;
        for (const string & w : words) {
            if (w.size() != pattern.size()) continue;
            if (test(w, pattern)) res.push_back(w);
        }
        return res;
    }
};