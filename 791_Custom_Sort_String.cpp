class Solution {
public:
    string customSortString(string S, string T) {
        vector<int> count(26, 0);
        for (char & c : T) {
            count[c - 'a']++;
        }
        string res;
        for (char & c : S) {
            while (count[c - 'a']) {
                res.push_back(c);
                count[c - 'a']--;
            }
        }
        for (char c = 'a'; c <= 'z'; c++) {
            while (count[c - 'a']) {
                res.push_back(c);
                count[c - 'a']--;
            }
        }
        return res;
    }
};