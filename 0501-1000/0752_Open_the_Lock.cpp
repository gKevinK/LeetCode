class Solution {
public:
    int openLock(vector<string>& deadends, string target) {
        if (target == "0000") return 0;
        unordered_set<string> s1 = { "0000" }, s2 = { target }, sd(deadends.begin(), deadends.end());
        if (sd.find("0000") != sd.end()) return -1;
        int r = 0;
        while (!s1.empty() && !s2.empty()) {
            r++;
            unordered_set<string> st, *ps1 = &s1, *ps2 = &s1;
            (s1.size() <= s2.size() ? ps2 : ps1) = &s2;
            for (const string & str : *ps1) {
                for (int i = 0; i < 8; i++) {
                    string s = str;
                    s[i / 2] += (i % 2 * 2 - 1);
                    if (s[i / 2] == '0' - 1) s[i / 2] = '9';
                    if (s[i / 2] == '9' + 1) s[i / 2] = '0';
                    if (sd.find(s) != sd.end()) continue;
                    if (ps2->find(s) != ps2->end()) return r;
                    st.insert(s);
                }
                sd.insert(str);
            }
            st.swap(*ps1);
        }
        return -1;
    }
};