class Solution {
public:
    string convertToTitle(int n) {
        string s;
        vector<char> v;
        do {
            v.push_back('A' + (--n) % 26);
            n /= 26;
        } while (n);
        for (int i = v.size() - 1; i >= 0; --i) {
            s.push_back(v[i]);
        }
        return s;
    }
};