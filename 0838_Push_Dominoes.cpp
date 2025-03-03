class Solution {
public:
    string pushDominoes(string dominoes) {
        string s = dominoes;
        for (int i = 0; i < s.size(); i++) {
            if (s[i] == 'L') {
                for (int j = i - 1; j >= 0 && s[j] == '.'; j--)
                    s[j] = 'L';
            } else if (s[i] == 'R') {
                int j = i + 1;
                while (j < s.size() && s[j] == '.') j++;
                if (j == s.size() || s[j] == 'R') {
                    for (int k = i + 1; k < j; k++)
                        s[k] = 'R';
                    i = j - 1;
                } else {
                    for (int k = 1; k * 2 < j - i; k++) {
                        s[i + k] = 'R';
                        s[j - k] = 'L';
                    }
                    i = j;
                }
            }
        }
        return s;
    }
};