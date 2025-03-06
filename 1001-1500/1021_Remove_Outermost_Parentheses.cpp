class Solution {
public:
    string removeOuterParentheses(string S) {
        string r;
        for (int i = 0, j = 1; i < S.size(); i = j++) {
            for (int n = 1; n; j++)
                n += S[j] == '(' ? 1 : -1;
            r += S.substr(i + 1, j - i - 2);
        }
        return r;
    }
};