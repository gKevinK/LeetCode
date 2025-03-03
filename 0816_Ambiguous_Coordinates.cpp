class Solution {
    vector<string> f(string s) {
        vector<string> r;
        if (s == "0" || s.front() != '0') {
            r.push_back(s);
        }
        if (s.back() != '0') {
            if (s.front() == '0') {
                r.push_back("0." + s.substr(1));
            } else {
                for (int i = 1; i < s.size(); ++i) {
                    r.push_back(s.substr(0, i) + "." + s.substr(i));
                }
            }
        }
        return r;
    }
public:
    vector<string> ambiguousCoordinates(string S) {
        vector<string> r;
        for (int i = 2; i < S.size() - 1; ++i) {
            auto va = f(S.substr(1, i - 1));
            auto vb = f(S.substr(i, S.size() - i - 1));
            for (auto & a : va) for (auto & b : vb) {
                r.push_back("(" + a + ", " + b + ")");
            }
        }
        return r;
    }
};