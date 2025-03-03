class Solution {
public:
    string reverseWords(string s) {
        for (int i = 0; i < s.size(); i++) {
            if (s[i] == ' ') continue;
            int j = i + 1;
            while (j < s.size() && s[j] != ' ')
                j++;
            for (int k1 = i, k2 = j - 1; k1 < k2; k1++, k2--)
                swap(s[k1], s[k2]);
            i = j;
        }
        return s;
    }
};