class Solution {
public:
    string longestWord(vector<string>& words) {
        sort(words.begin(), words.end(), [](auto & a, auto & b) {
            return a.size() != b.size() ? a.size() < b.size() : a < b; });
        unordered_set<string> s = { "" };
        string r;
        for (string & w : words) {
            if (s.find(w.substr(0, w.size() - 1)) != s.end()) {
                s.insert(w);
                if (w.size() > r.size())
                    r = w;
            }
        }
        return r;
    }
};