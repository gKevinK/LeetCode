class Solution {
    bool isVowel(char c) {
        if (c >= 'A' && c <= 'Z')
            c = c - 'A' + 'a';
        return c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u';
    }
public:
    string reverseVowels(string s) {
        int i = 0, j = s.size() - 1;
        while (i < j) {
            while (!isVowel(s[i]) && i < j) i++;
            while (!isVowel(s[j]) && i < j) j--;
            swap(s[i++], s[j--]);
        }
        return s;
    }
};