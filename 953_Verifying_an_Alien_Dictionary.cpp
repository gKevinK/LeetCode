class Solution {
    int comp(string & a, string & b, vector<int> & order) {
        int n = max(a.size(), b.size());
        for (int i = 0; i < n; i++) {
            if (i == a.size()) return -1;
            if (i == b.size()) return 1;
            if (order[a[i]] != order[b[i]]) return order[a[i]] - order[b[i]];
        }
        return 0;
    }
public:
    bool isAlienSorted(vector<string>& words, string order) {
        vector<int> v(128);
        for (int i = 0; i < 26; i++)
            v[order[i]] = i;
        for (int i = 1; i < words.size(); i++)
            if (comp(words[i - 1], words[i], v) > 0)
                return false;
        return true;
    }
};