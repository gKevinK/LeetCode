class Solution {
    unordered_map<char, int> map = {
        { 'q', 1 },
        { 'w', 1 },
        { 'e', 1 },
        { 'r', 1 },
        { 't', 1 },
        { 'y', 1 },
        { 'u', 1 },
        { 'i', 1 },
        { 'o', 1 },
        { 'p', 1 },
        { 'a', 2 },
        { 's', 2 },
        { 'd', 2 },
        { 'f', 2 },
        { 'g', 2 },
        { 'h', 2 },
        { 'j', 2 },
        { 'k', 2 },
        { 'l', 2 },
        { 'z', 3 },
        { 'x', 3 },
        { 'c', 3 },
        { 'v', 3 },
        { 'b', 3 },
        { 'n', 3 },
        { 'm', 3 },
    };
    
public:
    vector<string> findWords(vector<string>& words) {
        vector<string> res;
        for (const string & w : words) {
            int r = 0, f = 0;
            for (const char & c : w) {
                char c1 = (c <= 'Z' ? c - 'A' + 'a' : c);
                if (r == 0) r = map[c1];
                if (r != map[c1]) {
                    f = 1;
                    break;
                }
            }
            if (f == 0) res.push_back(w);
        }
        return res;
    }
};