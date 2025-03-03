class Solution {
public:
    string shiftingLetters(string S, vector<int>& shifts) {
        int s = 0;
        for (int i = S.size() - 1; i >= 0; i--) {
            s = (s + shifts[i]) % 26;
            S[i] = (S[i] - 'a' + s) % 26 + 'a';
        }
        return S;
    }
};