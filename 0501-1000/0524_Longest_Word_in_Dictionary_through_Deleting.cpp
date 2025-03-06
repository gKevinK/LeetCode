class Solution {
    bool f(string & s, string & w) {
        int i = 0, j = 0;
        for (; i < s.size() && j < w.size(); i++) {
            if (s[i] == w[j])
                j++;
        }
        return j == w.size();
    }
public:
    string findLongestWord(string s, vector<string>& d) {
        string r = "";
        for (string & w : d)
            if (f(s, w))
                if (w.size() > r.size() || (w.size() == r.size() && w < r))
                    r = w;
        return r;
    }
};