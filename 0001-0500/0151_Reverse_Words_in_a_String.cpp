class Solution {
public:
    void reverseWords(string &s) {
        int c = 0, wc = 0, j;
        for (int i = 0; i < s.size(); i = j + 1) {
            while (i < s.size() && s[i] == ' ') i++;
            if (i == s.size()) break;
            for (j = i; j < s.size() && s[j] != ' '; j++);
            for (int k = 0; k * 2 + 1 < j - i; k++) {
                swap(s[i + k], s[j - k - 1]);
            }
            if (wc) s[c++] = ' ';
            for (int k = 0; k < j - i; k++) {
                s[c++] = s[i + k];
            }
            wc++;
        }
        s.resize(c);
        for (int i = 0; i * 2 + 1 < s.size(); i++) {
            swap(s[i], s[s.size()-i-1]);
        }
    }
};