class Solution {
public:
    bool checkInclusion(string s1, string s2) {
        vector<int> n(26, 0);
        for (char & c : s1)
            n[c - 'a']++;
        for (int i = 0, j = 0; j < s2.size(); j++) {
            n[s2[j] - 'a']--;
            while (n[s2[j] - 'a'] < 0)
                n[s2[i++] - 'a']++;
            if (j - i + 1 == s1.size())
                return true;
        }
        return false;
    }
};