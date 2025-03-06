class Solution {
public:
    string reverseOnlyLetters(string S) {
        for (int i = 0, j = S.size(); i < j; i++, j--) {
            while (i < j && !isalpha(S[i])) i++;
            while (i < j && !isalpha(S[j])) j--;
            swap(S[i], S[j]);
        }
        return S;
    }
};