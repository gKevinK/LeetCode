class Solution {
public:
    bool wp2(map<char, string> &m, string &pattern, int pi, string &str, int si) {
        if (pi == pattern.size() && si >= str.size()) return true;
        else if (pi >= pattern.size() || si >= str.size()) return false;
        int si2 = si;
        while (si2 < str.size() && str[si2] != ' ') ++si2;
        string word = str.substr(si, si2 - si);
        if (m.find(pattern[pi]) == m.end()) {
            for (auto i = m.begin(); i != m.end(); ++i) if (i -> second.compare(word) == 0) return false;
            m[pattern[pi]] = word;
        } else if (m[pattern[pi]].compare(word) != 0) return false;
        return wp2(m, pattern, pi + 1, str, si2 + 1);
    }
    bool wordPattern(string pattern, string str) {
        map<char, string> m;
        return wp2(m, pattern, 0, str, 0);
    }
};