class Solution {
public:
    vector<string> removeComments(vector<string>& source) {
        vector<string> v;
        bool block = false, line = false;
        for (string & s : source) {
            if (!block) v.push_back("");
            bool c = block;
            line = false;
            for (int i = 0; i < s.size(); ++i) {
                if (block) {
                    if (i > 0 && s[i - 1] == '*' && s[i] == '/')
                        block = false;
                    continue;
                }
                if (line)
                    continue;
                if (i + 1 < s.size() && s[i] == '/' && s[i + 1] == '/') {
                    line = c = true;
                    continue;
                }
                if (i + 1 < s.size() && s[i] == '/' && s[i + 1] == '*') {
                    block = c = true;
                    i += 2;
                    continue;
                }
                v.back() += s[i];
            }
            if (c && v.back().empty())
                v.pop_back();
        }
        return v;
    }
};