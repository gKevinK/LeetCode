class Solution {
public:
    bool backspaceCompare(string S, string T) {
        string s1, s2;
        for (auto & c : S) {
            if (c == '#' && !s1.empty()) {
                s1.pop_back();
            } else if (c != '#') {
                s1.push_back(c);
            }
        }
        for (auto & c : T) {
            if (c == '#' && !s2.empty()) {
                s2.pop_back();
            } else if (c != '#') {
                s2.push_back(c);
            }
        }
        return s1 == s2;
    }
};